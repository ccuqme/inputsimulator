use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use evdev_rs::{
    DeviceWrapper,
    InputEvent,
    TimeVal,
    UInputDevice,
    UninitDevice,
    enums::{EventCode, EV_SYN, EV_KEY, EV_REL},
};

use crate::{
    config::{KeyBehaviorMode, ModifierBehaviorMode},
    constants::{
        SIMULATION_HOLD_DELAY_MS, 
        MAX_RETRIES,
        RETRY_DELAY_MS,
        MAX_DEVICE_INIT_RETRIES,
        DEVICE_INIT_RETRY_DELAY_MS,
    },
    error::{SimulatorError, Result},
};

fn retry<T, F>(mut operation: F, max_retries: u32, delay_ms: u64, log_fn: impl Fn(usize)) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut last_error = None;
    for attempt in 0..max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries - 1 {
                    log_fn((attempt + 1) as usize);
                    thread::sleep(Duration::from_millis(delay_ms));
                }
            }
        }
    }
    Err(last_error.unwrap())
}

fn write_event_with_retry(device: &UInputDevice, event: &InputEvent) -> Result<()> {
    retry(
        || {
            device.write_event(event)
                .map_err(|e| SimulatorError::KeySimulation(format!("Failed event: {:?}", e)).into())
        },
        MAX_RETRIES,
        RETRY_DELAY_MS,
        |attempt| {
            log::debug!("Write event attempt {} failed, retrying...", attempt);
        },
    )
    .map_err(|e| e)
}

fn write_key_events(device: &UInputDevice, keys: &[EventCode], value: i32, timeval: &TimeVal) -> Result<()> {
    for &key in keys {
        write_event_with_retry(device, &InputEvent::new(timeval, &key, value))?;
    }
    // Always sync after key events
    write_event_with_retry(device, &InputEvent::new(timeval, &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0))?;
    Ok(())
}

// Creates and configures a virtual input device with the specified key capabilities
fn setup_device(selected_keys: &Arc<Mutex<Vec<EventCode>>>) -> Result<UInputDevice> {
    let device = UninitDevice::new().unwrap();
    device.set_name("input_simulator");

    {
        let keys = selected_keys.lock().unwrap();
        
        // Always enable mouse buttons and basic mouse functionality
        device.enable(EventCode::EV_KEY(EV_KEY::BTN_LEFT)).unwrap();
        device.enable(EventCode::EV_KEY(EV_KEY::BTN_RIGHT)).unwrap();
        device.enable(EventCode::EV_KEY(EV_KEY::BTN_MIDDLE)).unwrap();
        device.enable(EventCode::EV_REL(EV_REL::REL_X)).unwrap();
        device.enable(EventCode::EV_REL(EV_REL::REL_Y)).unwrap();

        for &key in keys.iter() {
            device.enable(key).unwrap();
        }
    }

    let uinput_device = UInputDevice::create_from_device(&device)?;
    Ok(uinput_device)
}

fn setup_device_with_retry(selected_keys: &Arc<Mutex<Vec<EventCode>>>) -> Result<UInputDevice> {
    retry(
        || setup_device(selected_keys),
        MAX_DEVICE_INIT_RETRIES,
        DEVICE_INIT_RETRY_DELAY_MS,
        |attempt| {
            log::warn!("Device initialization attempt {} failed, retrying...", attempt);
        },
    )
    .map_err(|e| SimulatorError::DeviceInitialization(format!("Failed after {} retries: {:?}", MAX_DEVICE_INIT_RETRIES, e)).into())
}

// Initialize simulation keys
pub fn initialize_simulation_keys(
    app_data: &crate::config::AppData,
    selected_keys: &mut Vec<evdev_rs::enums::EventCode>,
    key_behavior: &mut crate::config::KeyBehaviorMode,
) {
    selected_keys.clear();
    *key_behavior = app_data.key_behavior;

    log::debug!("Initializing simulation with keys: {:?}", app_data.selected_keys);

    for raw in &app_data.selected_keys {
        if let Some(device_key) = crate::utils::key_utils::raw_key_to_device_keycode(raw) {
            if let Some(ev_key) = crate::utils::key_utils::keycode_to_evkey(device_key) {
                // Handle modifier keys based on modifier behavior setting
                if crate::utils::key_utils::is_modifier_key(raw) && 
                   app_data.modifier_behavior == crate::config::ModifierBehaviorMode::Click {
                    selected_keys.push(evdev_rs::enums::EventCode::EV_KEY(ev_key));
                } else {
                    selected_keys.push(evdev_rs::enums::EventCode::EV_KEY(ev_key));
                }
                log::debug!("Added key: {:?}", ev_key);
            }
        } else {
            log::warn!("Failed to map key: {}", raw);
        }
    }

    if selected_keys.is_empty() {
        log::warn!("No valid keys initialized for simulation");
    } else {
        log::info!("Simulation initialized with {} keys", selected_keys.len());
    }
}

// Main simulation loop that handles both click and hold modes
pub fn simulate_keys(
    running: Arc<Mutex<bool>>,
    interval_ms: Arc<Mutex<u64>>,
    selected_keys: Arc<Mutex<Vec<EventCode>>>,
    key_behavior: Arc<Mutex<KeyBehaviorMode>>,
    modifier_behavior: ModifierBehaviorMode,
) -> Result<()> {
    let uinput_device = setup_device_with_retry(&selected_keys)?;
    let timeval = TimeVal::new(0, 0);
    // Combine acquisitions for keys and mode.
    let (keys, mode) = {
        let keys = selected_keys.lock().unwrap().clone();
        let mode = *key_behavior.lock().unwrap();
        (keys, mode)
    };

    log::info!("Device initialized with keys: {:?}", keys);
    log::info!("Key behavior mode set to: {:?}", mode);

    // Initial sync
    write_event_with_retry(&uinput_device, &InputEvent::new(&timeval, &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0))?;

    match mode {
        KeyBehaviorMode::Hold => {
            thread::sleep(Duration::from_millis(SIMULATION_HOLD_DELAY_MS));
            
            // Press keys
            write_key_events(&uinput_device, &keys, 1, &timeval)?;

            while *running.lock().unwrap() {
                write_key_events(&uinput_device, &[], 0, &timeval)?;
            }

            // Release keys
            write_key_events(&uinput_device, &keys, 0, &timeval)?;
        }
        KeyBehaviorMode::Click => {
            if modifier_behavior == ModifierBehaviorMode::Click {
                // Separate modifier and non-modifier keys
                let (mod_keys, non_mod_keys): (Vec<EventCode>, Vec<EventCode>) = 
                    keys.iter().cloned().partition(|k| crate::utils::key_utils::is_modifier_evcode(k));

                while *running.lock().unwrap() {
                    let interval = *interval_ms.lock().unwrap();

                    // For each key sequence
                    for m in &mod_keys {
                        // Press and release modifier key first
                        write_key_events(&uinput_device, &[*m], 1, &timeval)?;
                        write_key_events(&uinput_device, &[*m], 0, &timeval)?;
                    }

                    // Then handle non-modifier keys
                    for nm in &non_mod_keys {
                        write_key_events(&uinput_device, &[*nm], 1, &timeval)?;
                        write_key_events(&uinput_device, &[*nm], 0, &timeval)?;
                    }

                    thread::sleep(Duration::from_millis(interval));
                }
            } else {
                while *running.lock().unwrap() {
                    let interval = *interval_ms.lock().unwrap();

                    // Press keys
                    write_key_events(&uinput_device, &keys, 1, &timeval)?;

                    // Release keys
                    write_key_events(&uinput_device, &keys, 0, &timeval)?;
                    thread::sleep(Duration::from_millis(interval));
                }
            }
        }
    }

    Ok(())
}
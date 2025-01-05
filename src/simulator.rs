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
    config::KeyBehaviorMode,
    constants::{
        SIMULATION_HOLD_DELAY_MS, 
        SIMULATION_HOLD_INTERVAL_MS,
        MAX_RETRIES,
        RETRY_DELAY_MS,
        MAX_DEVICE_INIT_RETRIES,
        DEVICE_INIT_RETRY_DELAY_MS,
    },
    error::{SimulatorError, Result},
};

fn write_event_with_retry(device: &UInputDevice, event: &InputEvent) -> Result<()> {
    let mut last_error = None;
    for attempt in 0..MAX_RETRIES {
        match device.write_event(event) {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_error = Some(e);
                if attempt < MAX_RETRIES - 1 {
                    log::debug!("Write event attempt {} failed, retrying...", attempt + 1);
                    thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
                }
            }
        }
    }
    Err(SimulatorError::KeySimulation(format!(
        "Failed after {} retries: {:?}", 
        MAX_RETRIES, 
        last_error.unwrap()
    )).into())
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
    let mut last_error = None;
    for attempt in 0..MAX_DEVICE_INIT_RETRIES {
        match setup_device(selected_keys) {
            Ok(device) => return Ok(device),
            Err(e) => {
                last_error = Some(e);
                if attempt < MAX_DEVICE_INIT_RETRIES - 1 {
                    log::warn!("Device initialization attempt {} failed, retrying...", attempt + 1);
                    thread::sleep(Duration::from_millis(DEVICE_INIT_RETRY_DELAY_MS));
                }
            }
        }
    }
    Err(SimulatorError::DeviceInitialization(format!(
        "Failed after {} retries: {:?}", 
        MAX_DEVICE_INIT_RETRIES, 
        last_error.unwrap()
    )).into())
}

// Main simulation loop that handles both click and hold modes
pub fn simulate_keys(
    running: Arc<Mutex<bool>>,
    interval_ms: Arc<Mutex<u64>>,
    selected_keys: Arc<Mutex<Vec<EventCode>>>,
    modifier_mode: Arc<Mutex<KeyBehaviorMode>>,
) -> Result<()> {
    let uinput_device = setup_device_with_retry(&selected_keys)?;
    let timeval = TimeVal::new(0, 0);
    let keys = selected_keys.lock().unwrap().clone();
    let mode = *modifier_mode.lock().unwrap();

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
                write_key_events(&uinput_device, &[], 0, &timeval)?; // Just sync
                thread::sleep(Duration::from_millis(SIMULATION_HOLD_INTERVAL_MS));
            }

            // Release keys
            write_key_events(&uinput_device, &keys, 0, &timeval)?;
        }
        KeyBehaviorMode::Click => {
            while *running.lock().unwrap() {
                let interval = *interval_ms.lock().unwrap();

                // Press keys
                write_key_events(&uinput_device, &keys, 1, &timeval)?;
                thread::sleep(Duration::from_millis(SIMULATION_HOLD_DELAY_MS));

                // Release keys
                write_key_events(&uinput_device, &keys, 0, &timeval)?;
                thread::sleep(Duration::from_millis(interval));
            }
        }
    }

    Ok(())
}
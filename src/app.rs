use cosmic::{
    app::{Core, Task},
    iced::{
        keyboard::{self, Key},
        Event, Subscription,
    },
    iced_core::SmolStr,
    Application, ApplicationExt, Element
};
use evdev_rs::enums::EventCode;
use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
    thread,
    time::{Instant},
};

use crate::{
    simulator::simulate_keys,
    config::{AppData, GlobalHotkey, KeyBehaviorMode, TempHotkeyState},
    utils::{from_cosmic_key, keycode_to_evkey, start_global_hotkey_listener}, // Direct imports from utils
    ui::View,
    constants::{DEFAULT_INTERVAL_MS},
};

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: cosmic::iced::keyboard::Modifiers,
}

impl KeyEvent {
    pub fn mouse_left() -> Self {
        Self {
            key: Key::Character(SmolStr::from("KEY_BTN_LEFT")),
            modifiers: cosmic::iced::keyboard::Modifiers::empty(),
        }
    }

    pub fn mouse_middle() -> Self {
        Self {
            key: Key::Character(SmolStr::from("KEY_BTN_MIDDLE")),
            modifiers: cosmic::iced::keyboard::Modifiers::empty(),
        }
    }

    pub fn mouse_right() -> Self {
        Self {
            key: Key::Character(SmolStr::from("KEY_BTN_RIGHT")),
            modifiers: cosmic::iced::keyboard::Modifiers::empty(),
        }
    }
}

// Main application struct managing UI state and background threads
pub struct InputSimulatorApp {
    running: Arc<Mutex<bool>>,
    interval_ms: Arc<Mutex<u64>>,
    core: Core,
    app_data: Arc<Mutex<AppData>>,
    capturing: Arc<Mutex<bool>>,
    selected_keys: Arc<Mutex<Vec<EventCode>>>,
    modifier_mode: Arc<Mutex<KeyBehaviorMode>>,  
    previous_hotkey_state: Arc<Mutex<bool>>,
    last_toggle_time: Arc<Mutex<Option<Instant>>>,
    capturing_hotkey: Arc<Mutex<bool>>,
}

impl Default for InputSimulatorApp {
    fn default() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            interval_ms: Arc::new(Mutex::new(DEFAULT_INTERVAL_MS)),
            core: Core::default(),
            app_data: Arc::new(Mutex::new(AppData {
                captured_keys: Vec::new(),
                selected_keys: Vec::new(),
                global_keybind: GlobalHotkey::default(),
                interval_ms: 100,
                modifier_mode: KeyBehaviorMode::Click,
                capturing_global_hotkey: false,
                captured_global_hotkey: None,
                temp_hotkey: TempHotkeyState::default(),
            })),
            capturing: Arc::new(Mutex::new(false)),
            selected_keys: Arc::new(Mutex::new(Vec::new())),
            modifier_mode: Arc::new(Mutex::new(KeyBehaviorMode::Click)),
            previous_hotkey_state: Arc::new(Mutex::new(false)),
            last_toggle_time: Arc::new(Mutex::new(None)),
            capturing_hotkey: Arc::new(Mutex::new(false)),
        }
    }
}

impl Application for InputSimulatorApp {
    type Message = Message;
    type Executor = cosmic::executor::Default;
    type Flags = ();

    const APP_ID: &'static str = "input_simulator";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = Self {
            core,
            ..Self::default()
        };

        if let Some(id) = app.core.main_window_id() {
            let _ = app.set_window_title("Input Simulator".to_string(), id);
        } else {
            log::error!("Failed to retrieve the main window ID");
        }

        app.load_app_data();
        app.start_global_hotkey_listener();
        
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::ToggleRunning => {
                let mut running = self.running.lock().unwrap();
                *running = !*running;

                if *running {
                    self.start_simulation();
                }
            }
            Message::SetInterval(interval) => {
                {
                    let mut app_data = self.app_data.lock().unwrap();
                    log::debug!("Setting interval: {} ms", interval);
                    app_data.interval_ms = interval;
                }
                *self.interval_ms.lock().unwrap() = interval;
            }
            Message::SetIntervalAndSave(interval) => {
                {
                    let mut app_data = self.app_data.lock().unwrap();
                    log::info!("Updating interval from {} ms to {} ms", app_data.interval_ms, interval);
                    app_data.interval_ms = interval;
                }
                *self.interval_ms.lock().unwrap() = interval;
                let _ = self.save_app_data();
            }
            Message::UpdateInterval(input) => {
                if let Ok(value) = input.parse::<u64>() {
                    {
                        let mut app_data = self.app_data.lock().unwrap();
                        log::info!("Manually setting interval from {} ms to {} ms", app_data.interval_ms, value);
                        app_data.interval_ms = value;
                    }
                    *self.interval_ms.lock().unwrap() = value;
                    let _ = self.save_app_data();
                } else {
                    log::warn!("Invalid interval input: {}", input);
                }
            }
            Message::CaptureKeys => {
                let mut capturing_lock = self.capturing.lock().unwrap();
                *capturing_lock = true;
                
                let mut app_data_lock = self.app_data.lock().unwrap();
                app_data_lock.captured_keys.clear();
                log::info!("Started key capture mode");
            }
            Message::AddKey(key_event) => {
                let mut app_data = self.app_data.lock().unwrap();
                if app_data.capturing_global_hotkey {
                    let temp_hotkey = &mut app_data.temp_hotkey;
                    
                    // Update modifiers first
                    temp_hotkey.modifiers.ctrl = key_event.modifiers.control();
                    temp_hotkey.modifiers.alt = key_event.modifiers.alt();
                    temp_hotkey.modifiers.shift = key_event.modifiers.shift();
                    temp_hotkey.modifiers.super_key = key_event.modifiers.logo();

                    // Skip updating the key if it's a modifier key
                    let key_str = format!("{:?}", key_event.key);
                    if !key_str.contains("Control") && 
                       !key_str.contains("Alt") && 
                       !key_str.contains("Shift") && 
                       !key_str.contains("Logo") {
                        log::debug!("Adding key to hotkey: {}", key_str);
                        // For Named keys (like function keys), extract just the name
                        if key_str.starts_with("Named(") {
                            let clean_key = key_str.trim_start_matches("Named(")
                                                 .trim_end_matches(")")
                                                 .to_string();
                            temp_hotkey.key = Some(clean_key);
                        } else {
                            // Extract raw key name without formatting
                            let clean_key = key_str.trim_start_matches("KEY_")
                                                   .to_string();
                            temp_hotkey.key = Some(clean_key);
                        }
                    }
                } else if *self.capturing.lock().unwrap() {
                    if !app_data.captured_keys.contains(&key_event.key) {
                        log::debug!("Captured new key: {:?}", key_event.key);
                        app_data.captured_keys.push(key_event.key);
                    }
                }
            }
            Message::FinalizeKeys => {
                let captured_keys = {
                    let app_data = self.app_data.lock().unwrap();
                    app_data.captured_keys.clone()
                };

                log::info!("Finalizing captured keys: {:?}", captured_keys);
                let converted_keys: Vec<Key> = captured_keys
                    .iter()
                    .filter_map(|key| {
                        from_cosmic_key(key.clone())
                            .and_then(keycode_to_evkey)
                            .map(|ev_key| {
                                Key::Character(SmolStr::from(format!("KEY_{:?}", ev_key)))
                            })
                    })
                    .collect();

                {
                    let mut app_data = self.app_data.lock().unwrap();
                    app_data.selected_keys = converted_keys.clone();
                    *self.capturing.lock().unwrap() = false;
                }
                log::info!("Successfully saved captured keys: {:?}", converted_keys);
                let _ = self.save_app_data();
            }
            Message::CancelCapture => {
                let mut capturing_lock = self.capturing.lock().unwrap();
                *capturing_lock = false;
                
                let mut app_data = self.app_data.lock().unwrap();
                app_data.captured_keys.clear();
            }
            Message::UpdateKeyBehaviorMode(mode) => {
                {
                    let mut app_data = self.app_data.lock().unwrap();
                    app_data.modifier_mode = mode;
                    log::info!("Key behavior mode updated to: {:?}", mode);
                }
            
                if let Err(e) = self.save_app_data() {
                    log::error!("Failed to save app data: {:?}", e);
                }
            }
            Message::CaptureGlobalHotkey => {
                let mut capturing_hotkey = self.capturing_hotkey.lock().unwrap();
                *capturing_hotkey = true;
                
                let mut app_data = self.app_data.lock().unwrap();
                app_data.capturing_global_hotkey = true;
                app_data.temp_hotkey = TempHotkeyState::default();
            }
            Message::FinalizeGlobalHotkey => {
                {
                    let mut app_data = self.app_data.lock().unwrap();
                    if let Some(key) = &app_data.temp_hotkey.key {
                        let modifiers = &app_data.temp_hotkey.modifiers;
                        let hotkey_desc = format!(
                            "{}{}{}{}{}",
                            if modifiers.ctrl { "Ctrl+" } else { "" },
                            if modifiers.alt { "Alt+" } else { "" },
                            if modifiers.shift { "Shift+" } else { "" },
                            if modifiers.super_key { "Super+" } else { "" },
                            key
                        );
                        log::info!("Setting new global hotkey: {}", hotkey_desc);
                        
                        app_data.global_keybind = GlobalHotkey {
                            key: key.clone(),
                            modifiers: app_data.temp_hotkey.modifiers,
                        };
                    }
                    app_data.capturing_global_hotkey = false;
                    *self.capturing_hotkey.lock().unwrap() = false;
                }
                let _ = self.save_app_data();
            }
            Message::CancelGlobalHotkey => {
                let mut capturing_hotkey = self.capturing_hotkey.lock().unwrap();
                *capturing_hotkey = false;
                
                let mut app_data = self.app_data.lock().unwrap();
                app_data.capturing_global_hotkey = false;
                app_data.captured_global_hotkey = None;
            }
            Message::Noop => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let app_data = self.app_data.lock().unwrap();
        let view = View::new(
            *self.running.lock().unwrap(),
            *self.interval_ms.lock().unwrap() as f64,
            app_data,
            *self.capturing.lock().unwrap(),
            *self.capturing_hotkey.lock().unwrap(),
        );
        
        view.build()
    }

    fn on_close_requested(&self, _id: cosmic::iced::window::Id) -> Option<Self::Message> {
        None
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::event::listen_with(|event, _status, _context| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { 
                    key,
                    modifiers,
                    ..
                }) => {
                    Some(Message::AddKey(KeyEvent { 
                        key: key.clone(), 
                        modifiers 
                    }))
                }
                _ => None,
            }
        })
    }
}

impl InputSimulatorApp {
    // Starts the input simulation thread with current configuration
    fn start_simulation(&self) {
        let running = Arc::clone(&self.running);
        let interval_ms = Arc::clone(&self.interval_ms);
        let selected_keys = Arc::clone(&self.selected_keys);
        let modifier_mode = Arc::clone(&self.modifier_mode);
    
        // Update selected_keys and modifier_mode from app_data
        {
            let app_data = self.app_data.lock().unwrap();
            let mut keys = self.selected_keys.lock().unwrap();
            *self.modifier_mode.lock().unwrap() = app_data.modifier_mode;
            keys.clear();
    
            log::debug!("Starting simulation with keys: {:?}", app_data.selected_keys);
    
            for key in &app_data.selected_keys {
                if let Key::Character(key_str) = key {
                    // Remove duplicate KEY_ prefixes and process
                    let clean_key = key_str.replace("KEY_KEY_", "KEY_");
                    if let Some(device_key) = from_cosmic_key(Key::Character(SmolStr::from(clean_key.clone()))) {
                        if let Some(ev_key) = keycode_to_evkey(device_key) {
                            keys.push(EventCode::EV_KEY(ev_key));
                            log::debug!("Added key for simulation: {:?}", ev_key);
                        } else {
                            log::warn!("Failed to convert to ev_key: {:?}", clean_key);
                        }
                    } else {
                        log::warn!("Failed to convert cosmic key: {:?}", clean_key);
                    }
                }
            }
    
            log::debug!("Final simulation keys: {:?}", *keys);
    
            // Check for empty keys
            if keys.is_empty() {
                log::warn!("No valid keys for simulation, skipping start.");
                *self.running.lock().unwrap() = false;
                return;
            }
        }
    
        // Spawn the simulation thread if keys are valid
        thread::spawn(move || {
            if let Err(e) = simulate_keys(running, interval_ms, selected_keys, modifier_mode) {
                log::error!("Failed to simulate keys: {}", e);
            }
        });
    }

    // Persists application state to disk
    fn save_app_data(&self) -> std::io::Result<()> {
        let mut app_data = self.app_data.lock().unwrap();
        // Ensure global_keybind is in the correct format if empty
        if app_data.global_keybind.key.is_empty() {
            app_data.global_keybind.key = "Named(F8)".to_string();
        }
        
        log::debug!("Saving app data - Selected keys: {:?}, Global hotkey: {:?}", 
            app_data.selected_keys, app_data.global_keybind);
            
        let json = serde_json::to_string(&*app_data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        std::fs::write("app_data.json", json)?;
        log::info!("Successfully saved app data");
        Ok(())
    }

    // Loads application state from disk
    fn load_app_data(&mut self) {
        match File::open("app_data.json") {
            Ok(mut file) => {
                let mut json = String::new();
                if file.read_to_string(&mut json).is_ok() {
                    match serde_json::from_str::<AppData>(&json) {
                        Ok(data) => {
                            log::info!("Loaded app data: {:?}", data.selected_keys);
                            self.app_data = Arc::new(Mutex::new(data));
                        }
                        Err(e) => log::error!("Failed to parse app data: {}", e),
                    }
                }
            }
            Err(e) => log::warn!("Failed to open app data: {}", e),
        }
    }

    // Monitors global hotkeys in a background thread
    fn start_global_hotkey_listener(&self) {
        start_global_hotkey_listener(
            Arc::clone(&self.running),
            Arc::clone(&self.interval_ms),
            Arc::clone(&self.selected_keys),
            Arc::clone(&self.modifier_mode),
            Arc::clone(&self.previous_hotkey_state),
            Arc::clone(&self.last_toggle_time),
            Arc::clone(&self.app_data),
        );
    }
}

// Messages for UI state updates and user interactions
#[derive(Debug, Clone)]
pub enum Message {
    ToggleRunning,
    SetInterval(u64),
    CaptureKeys,
    AddKey(KeyEvent),
    FinalizeKeys,
    UpdateInterval(String),
    Noop,
    CancelCapture,
    SetIntervalAndSave(u64),
    UpdateKeyBehaviorMode(KeyBehaviorMode),
    CaptureGlobalHotkey,
    FinalizeGlobalHotkey,
    CancelGlobalHotkey,
}
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
    time::Instant,
};

use crate::{
    simulator::simulate_keys,
    config::{AppData, GlobalHotkey, KeyBehaviorMode, ModifierBehaviorMode, HoldBehaviorMode, TempHotkeyState},
    utils::start_global_hotkey_listener, 
    ui::View,
    constants::DEFAULT_INTERVAL_MS,
    utils::persistence::save_app_data,
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
    key_behavior: Arc<Mutex<KeyBehaviorMode>>,  
    previous_hotkey_state: Arc<Mutex<bool>>,
    last_toggle_time: Arc<Mutex<Option<Instant>>>,
    capturing_hotkey: Arc<Mutex<bool>>,
    settings_panel_open: Arc<Mutex<bool>>,
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
                key_behavior: KeyBehaviorMode::Click,
                modifier_behavior: ModifierBehaviorMode::Click,
                hold_behavior: HoldBehaviorMode::default(),
                settings_panel_open: true,
                capturing_global_hotkey: false,
                temp_hotkey: TempHotkeyState::default(),
            })),
            capturing: Arc::new(Mutex::new(false)),
            selected_keys: Arc::new(Mutex::new(Vec::new())),
            key_behavior: Arc::new(Mutex::new(KeyBehaviorMode::Click)),
            previous_hotkey_state: Arc::new(Mutex::new(false)),
            last_toggle_time: Arc::new(Mutex::new(None)),
            capturing_hotkey: Arc::new(Mutex::new(false)),
            settings_panel_open: Arc::new(Mutex::new(true)),
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
        
        // Sync the settings_panel_open state with the loaded app_data
        {
            let app_data = app.app_data.lock().unwrap();
            let mut settings_panel_open = app.settings_panel_open.lock().unwrap();
            *settings_panel_open = app_data.settings_panel_open;
        }
        
        app.start_global_hotkey_listener();
        
        // Set initial window size based on settings panel state
        let resize_task = app.set_initial_window_size();
        
        (app, resize_task)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::ToggleRunning                => self.handle_toggle_running(),
            Message::SetInterval(interval)         => self.handle_set_interval(interval),
            Message::SetIntervalAndSave(interval)    => self.handle_set_interval_and_save(interval),
            Message::UpdateInterval(input)         => self.handle_update_interval(input),
            Message::CaptureKeys                   => self.handle_capture_keys(),
            Message::AddKey(key_event)             => self.handle_add_key(key_event),
            Message::FinalizeKeys                  => self.handle_finalize_keys(),
            Message::UpdateModifierBehaviorMode(mode) => self.handle_update_modifier_behavior_mode(mode),
            Message::CancelCapture                 => self.handle_cancel_capture(),
            Message::UpdateKeyBehaviorMode(mode)   => self.handle_update_key_behavior_mode(mode),
            Message::UpdateHoldBehaviorMode(mode)  => self.handle_update_hold_behavior_mode(mode),
            Message::CaptureGlobalHotkey           => self.handle_capture_global_hotkey(),
            Message::FinalizeGlobalHotkey          => self.handle_finalize_global_hotkey(),
            Message::CancelGlobalHotkey            => self.handle_cancel_global_hotkey(),
            Message::ToggleSettingsPanel           => {
                let panel_open = {
                    let mut settings_panel_open = self.settings_panel_open.lock().unwrap();
                    *settings_panel_open = !*settings_panel_open;
                    *settings_panel_open
                };
                
                self.update_state(|app_data| {
                    app_data.settings_panel_open = panel_open;
                });
                
                return self.resize_window(panel_open);
            },
            Message::RefreshUiState                => {},
            Message::Noop                          => {},
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
            *self.settings_panel_open.lock().unwrap(),
        );
        
        view.build()
    }

    fn on_close_requested(&self, _id: cosmic::iced::window::Id) -> Option<Self::Message> {
        None
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
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
            }),
            timer_subscription(250)
        ])
    }
}

impl InputSimulatorApp {
    // Helper function to spawn a simulation thread with proper Arc cloning
    fn spawn_simulation_thread(
        running: Arc<Mutex<bool>>,
        interval_ms: Arc<Mutex<u64>>,
        selected_keys: Arc<Mutex<Vec<EventCode>>>,
        key_behavior: Arc<Mutex<KeyBehaviorMode>>,
        app_data: Arc<Mutex<AppData>>,
    ) {
        // Clone Arcs before moving them into the closure
        let running_inner = Arc::clone(&running);
        let interval_ms_inner = Arc::clone(&interval_ms);
        let selected_keys_inner = Arc::clone(&selected_keys);
        let key_behavior_inner = Arc::clone(&key_behavior);
        let app_data_inner = Arc::clone(&app_data);

        thread::spawn(move || {
            let (mod_behavior, hold_behavior) = {
                let ad = app_data_inner.lock().unwrap();
                (ad.modifier_behavior, ad.hold_behavior)
            };
            if let Err(e) = simulate_keys(
                running_inner,
                interval_ms_inner,
                selected_keys_inner,
                key_behavior_inner,
                mod_behavior,
                hold_behavior,
            ) {
                log::error!("Failed to simulate keys: {}", e);
            }
        });
    }

    // Resize window based on the settings panel state
    fn resize_window(&self, panel_open: bool) -> Task<Message> {
        if let Some(window_id) = self.core.main_window_id() {
            let size = if panel_open {
                crate::ui::WINDOW_SIZE_WITH_PANEL
            } else {
                crate::ui::WINDOW_SIZE_WITHOUT_PANEL
            };
            
            log::info!("Resizing window to {}", if panel_open { 
                format!("default ({}x{})", size.0, size.1) 
            } else { 
                format!("small ({}x{})", size.0, size.1) 
            });
            
            return cosmic::iced::window::resize(window_id, cosmic::iced::Size::new(size.0, size.1));
        }
        
        Task::none()
    }
    // Starts the input simulation thread with current configuration
    fn start_simulation(&self) {
        let running = Arc::clone(&self.running);
        let interval_ms = Arc::clone(&self.interval_ms);
        let selected_keys = Arc::clone(&self.selected_keys);
        let key_behavior = Arc::clone(&self.key_behavior);
        let app_data = Arc::clone(&self.app_data);

        {
            let app_data_guard = app_data.lock().unwrap();
            let mut keys_lock = selected_keys.lock().unwrap();
            let mut behavior_lock = key_behavior.lock().unwrap();
            crate::simulator::initialize_simulation_keys(&app_data_guard, &mut keys_lock, &mut behavior_lock);
            if keys_lock.is_empty() {
                log::warn!("No valid keys for simulation, skipping start.");
                *running.lock().unwrap() = false;
                return;
            }
        }

        Self::spawn_simulation_thread(running, interval_ms, selected_keys, key_behavior, app_data);
    }

    // Persists application state to disk using the unified persistence function.
    fn save_app_data(&self) -> crate::error::Result<()> {
        let mut app_data = self.app_data.lock().unwrap();
        
        // The normalization and defaulting are now handled in the persistence module.
        save_app_data(&mut app_data)?;
        log::info!("Successfully saved app data");
        Ok(())
    }
    
    fn set_initial_window_size(&mut self) -> Task<Message> {
        let settings_panel_open = self.app_data.lock().unwrap().settings_panel_open;
        self.resize_window(settings_panel_open)
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
                            *self.interval_ms.lock().unwrap() = data.interval_ms;
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
        // Capture all necessary state with a single clone at the beginning
        let running = Arc::clone(&self.running);
        let interval_ms = Arc::clone(&self.interval_ms);
        let selected_keys = Arc::clone(&self.selected_keys);
        let key_behavior = Arc::clone(&self.key_behavior);
        let app_data = Arc::clone(&self.app_data);
        let previous_hotkey_state = Arc::clone(&self.previous_hotkey_state);
        let last_toggle_time = Arc::clone(&self.last_toggle_time);

        start_global_hotkey_listener(
            Arc::clone(&running),
            Arc::clone(&interval_ms),
            Arc::clone(&selected_keys),
            Arc::clone(&key_behavior),
            Arc::clone(&previous_hotkey_state),
            Arc::clone(&last_toggle_time),
            Arc::clone(&app_data),
            Arc::new(move || {
                log::info!("Global hotkey pressed.");
                let mut running_lock = running.lock().unwrap();
                *running_lock = !*running_lock;
                if *running_lock {
                    // Initialize simulation keys from latest app_data.
                    {
                        let app_data_guard = app_data.lock().unwrap();
                        let mut keys_lock = selected_keys.lock().unwrap();
                        let mut behavior_lock = key_behavior.lock().unwrap();
                        crate::simulator::initialize_simulation_keys(&app_data_guard, &mut keys_lock, &mut behavior_lock);
                        if keys_lock.is_empty() {
                            log::warn!("No valid keys for simulation, skipping simulation start.");
                            *running_lock = false;
                            return;
                        }
                    }

                    // Use the static helper function to spawn the simulation thread
                    Self::spawn_simulation_thread(
                        Arc::clone(&running),
                        Arc::clone(&interval_ms),
                        Arc::clone(&selected_keys),
                        Arc::clone(&key_behavior),
                        Arc::clone(&app_data)
                    );
                }
            })
        );
    }

    // Helper: Toggle running state and start simulation if needed.
    fn handle_toggle_running(&mut self) {
        let mut running = self.running.lock().unwrap();
        *running = !*running;
        if *running {
            self.start_simulation();
        }
    }

    // New helper function for state updates that automatically saves changes
    fn update_state<F, T>(&self, update_fn: F) -> T 
    where
        F: FnOnce(&mut AppData) -> T
    {
        let result = {
            let mut app_data = self.app_data.lock().unwrap();
            update_fn(&mut app_data)
        };
        let _ = self.save_app_data();
        result
    }

    // Helper for interval updates
    fn set_interval_internal(&mut self, interval: u64, _save: bool) {
        self.update_state(|app_data| {
            log::info!("Updating interval from {} ms to {} ms", app_data.interval_ms, interval);
            app_data.interval_ms = interval;
        });
        *self.interval_ms.lock().unwrap() = interval;
    }

    // Updated handlers using the new helpers
    fn handle_set_interval(&mut self, interval: u64) {
        self.set_interval_internal(interval, false);
    }

    fn handle_set_interval_and_save(&mut self, interval: u64) {
        self.set_interval_internal(interval, true);
    }

    fn handle_update_interval(&mut self, input: String) {
        if let Ok(value) = input.parse::<u64>() {
            self.set_interval_internal(value, true);
        } else {
            log::warn!("Invalid interval input: {}", input);
        }
    }

    fn handle_update_key_behavior_mode(&mut self, mode: KeyBehaviorMode) {
        self.update_state(|app_data| {
            app_data.key_behavior = mode;
            log::info!("Key behavior mode updated to: {:?}", mode);
        });
    }

    fn handle_update_modifier_behavior_mode(&mut self, mode: ModifierBehaviorMode) {
        self.update_state(|app_data| {
            app_data.modifier_behavior = mode;
            log::info!("Modifier behavior mode updated to: {:?}", mode);
        });
    }

    fn handle_update_hold_behavior_mode(&mut self, mode: HoldBehaviorMode) {
        self.update_state(|app_data| {
            app_data.hold_behavior = mode;
            log::info!("Hold behavior mode updated to: {:?}", mode);
        });
    }

    fn handle_capture_keys(&mut self) {
        *self.capturing.lock().unwrap() = true;
        self.update_state(|app_data| {
            app_data.captured_keys.clear();
            log::info!("Started key capture mode");
        });
    }

    fn handle_finalize_keys(&mut self) {
        *self.capturing.lock().unwrap() = false;
        self.update_state(|app_data| {
            log::info!("Finalizing captured keys: {:?}", app_data.captured_keys);
            app_data.selected_keys = app_data.captured_keys.clone();
        });
    }

    fn handle_capture_global_hotkey(&mut self) {
        *self.capturing_hotkey.lock().unwrap() = true;
        self.update_state(|app_data| {
            app_data.capturing_global_hotkey = true;
            app_data.temp_hotkey = TempHotkeyState::default();
        });
    }

    fn handle_finalize_global_hotkey(&mut self) {
        *self.capturing_hotkey.lock().unwrap() = false;
        self.update_state(|app_data| {
            if let Some(key) = &app_data.temp_hotkey.key {
                let normalized = crate::utils::key_utils::normalize_key(key);
                let modifiers = &app_data.temp_hotkey.modifiers;
                let hotkey_desc = format!(
                    "{}{}{}{}{}",
                    if modifiers.ctrl { "Ctrl+" } else { "" },
                    if modifiers.alt { "Alt+" } else { "" },
                    if modifiers.shift { "Shift+" } else { "" },
                    if modifiers.super_key { "Super+" } else { "" },
                    normalized
                );
                log::info!("Setting new global hotkey: {}", hotkey_desc);
                app_data.global_keybind = GlobalHotkey {
                    key: normalized,
                    modifiers: app_data.temp_hotkey.modifiers,
                };
            }
            app_data.capturing_global_hotkey = false;
        });
    }

    // Helper: Process new key events.
    fn handle_add_key(&mut self, key_event: KeyEvent) {
        let raw = format!("{:?}", key_event.key);
        let normalized = crate::utils::key_utils::normalize_key(raw.as_str());
        let is_capturing = *self.capturing.lock().unwrap();
        
        self.update_state(|app_data| {
            if app_data.capturing_global_hotkey {
                app_data.temp_hotkey.key = Some(normalized);
                let temp_hotkey = &mut app_data.temp_hotkey;
                temp_hotkey.modifiers.ctrl = key_event.modifiers.control();
                temp_hotkey.modifiers.alt = key_event.modifiers.alt();
                temp_hotkey.modifiers.shift = key_event.modifiers.shift();
                temp_hotkey.modifiers.super_key = key_event.modifiers.logo();
            } else if is_capturing {
                if !app_data.captured_keys.contains(&normalized) {
                    log::debug!("Captured new key: {}", normalized);
                    app_data.captured_keys.push(normalized);
                }
            }
        });
    }

    // Helper: Cancel key capture.
    fn handle_cancel_capture(&mut self) {
        *self.capturing.lock().unwrap() = false;
        self.update_state(|app_data| {
            app_data.captured_keys.clear();
        });
    }

    // Helper: Cancel global hotkey capture.
    fn handle_cancel_global_hotkey(&mut self) {
        *self.capturing_hotkey.lock().unwrap() = false;
        self.update_state(|app_data| {
            app_data.capturing_global_hotkey = false;
        });
    }
}

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
    UpdateModifierBehaviorMode(ModifierBehaviorMode),
    UpdateHoldBehaviorMode(HoldBehaviorMode),
    ToggleSettingsPanel,
    RefreshUiState,
}

// Timer subscription that periodically sends a message to refresh the UI state
fn timer_subscription(interval_ms: u64) -> Subscription<Message> {
    use cosmic::iced::futures::stream;

    Subscription::run_with_id(
        "ui-refresh-timer",
        stream::unfold(
            std::time::Instant::now(),
            move |_last_tick| {               
                async move {
                    smol::Timer::after(std::time::Duration::from_millis(interval_ms)).await;
                    Some((Message::RefreshUiState, std::time::Instant::now()))
                }
            }
        )
    )
}
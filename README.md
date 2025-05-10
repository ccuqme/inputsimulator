# Linux AutoClicker for Keyboard and Mouse
![image](https://github.com/user-attachments/assets/dff24dc2-04a4-4fbe-b5df-4fe36d6a4a40)

This tool automates repetitive key presses and mouse clicks at customizable intervals. You can easily set up specific keys or mouse buttons to simulate and toggle the simulation on or off using a global hotkey.

## Features

- **Configurable Key Behavior**:
  - Hold: 
    - Continuous: Hold all keys down simultaneously until stopped.
    - Cycle: Press each key sequentially, holding each for the specified interval.
  - Click: Simulate keys being pressed and released repeatedly at a set interval.
- **Modifier Behavior**:
  - Click: Press and release modifier keys separately from regular keys.
  - Hold: Press modifier keys together with regular keys.
- **Global Hotkeys**:
  - Assign a hotkey to toggle the simulation on or off (default: `F8`).

## Compatibility

- **Platforms**: Made for Linux, untested and unsupported on anything else.
- **Desktop Environments**: Tested on KDE Plasma 6.3.4 and COSMIC Desktop Alpha 7.1.

## Known Issues

- Global hotkeys **do not work** with pure Wayland applications on the COSMIC Desktop, but they work fine with Xwayland applications (e.g., Steam and games running through Proton).
- The application starts with a phantom winit window. This is a minor issue that may be addressed in the future.

## Building from Source

To build the application from source, you need to have Rust and Cargo installed. Follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/ccuqme/inputsimulator.git
    cd inputsimulator
    ```

2. Build the application:
    ```sh
    cargo build --release
    ```

3. The built executable will be located in the `target/release` directory.

## Dependencies

- [libcosmic](https://github.com/pop-os/libcosmic)
- [evdev-rs](https://crates.io/crates/evdev-rs)
- [device_query](https://crates.io/crates/device_query)
- [serde](https://crates.io/crates/serde)
- [thiserror](https://crates.io/crates/thiserror)
- [log](https://crates.io/crates/log)
- [regex](https://crates.io/crates/regex)
- [smol](https://crates.io/crates/smol)

## Contributing

This is a personal project primarily made for my own use, but suggestions and contributions are always welcome! If you have ideas, encounter bugs, or want to improve the app, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Third-Party Licenses

This application relies on third-party dependencies, which may be licensed differently. For details, refer to the [THIRD_PARTY_LICENSES.txt](THIRD_PARTY_LICENSES.txt) file.

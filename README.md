![image](https://github.com/user-attachments/assets/b6aba3b6-d9ee-47ba-a204-8d3c7e01f572)

# AutoClicker for Keyboard and Mouse

This application simulates repeated key presses and mouse clicks at a specified interval. You can configure which keys or mouse buttons to simulate and use a global hotkey to toggle the simulation on or off. 

## Features

- **Configurable Key Behavior**:
  - Hold: Simulate keys being pressed continuously.
  - Click: Simulate keys being pressed and released repeatedly at a set interval.
- **Global Hotkeys**:
  - Assign a hotkey to toggle the simulation on or off (default: `F8`).

## Compatibility

- **Platforms**: Made for Linux, but there is a slight chance it might work on macOS (although unsupported) due to its `evdev` support.
- **Desktop Environments**: Tested on KDE Plasma 6.2.4 and COSMIC Desktop Alpha 4.

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
- [lazy_static](https://crates.io/crates/lazy_static)
- [thiserror](https://crates.io/crates/thiserror)
- [log](https://crates.io/crates/log)

## Contributing

This is a personal project primarily made for my own use, but suggestions and contributions are always welcome! If you have ideas, encounter bugs, or want to improve the app, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Third-Party Licenses

This application relies on third-party dependencies, which may be licensed differently. For details, refer to the [THIRD_PARTY_LICENSES.txt](THIRD_PARTY_LICENSES.txt) file.

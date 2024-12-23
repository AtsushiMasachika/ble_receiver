# BLE Receiver

A simple BLE (Bluetooth Low Energy) receiver application built in Rust. This project demonstrates how to scan for nearby BLE devices, retrieve device information, and handle adapter events using the [`bluer`](https://crates.io/crates/bluer) library.

## Features

- Scan for BLE devices in the vicinity
- Retrieve device details:
  - Name
  - Address
  - RSSI (signal strength)
  - Pairing status
- Handle adapter events:
  - Device added
  - Device removed
- Built with asynchronous Rust using the [`tokio`](https://crates.io/crates/tokio) runtime

## Requirements

- **Rust**: Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs).
- **BlueZ**: The application uses the BlueZ stack for Bluetooth communication. Install it on Linux systems using:
  ```bash
  sudo apt install bluez
  ```
  
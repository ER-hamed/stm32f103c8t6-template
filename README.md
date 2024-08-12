# 🦀 Rust on STM32F103C8T6 Template

This repository provides a ready-to-use template for running Rust projects on the STM32F103C8T6 microcontroller.

## 📋 Specifications

- Programming Language: Rust 🦀
- Target Hardware: STM32F103C8T6 (Blue Pill)
- Development Environment: Based on embedded-hal

## 🚀 Features

- Initial configuration for STM32F103C8T6
- RTT support for debugging
- Cargo.toml with initial settings and dependencies

## 🛠️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)
- [probe-run](https://probe.rs/docs/getting-started/installation/)
- [stm32-rs](https://github.com/stm32-rs)

## 🔧 How to Use

1. Clone this repository:
``` sh
git clone https://github.com/your-username/rust-stm32f103c8t6-template.git && cd stm32f103c8t6-template
```
2. Build the project:
``` sh
cargo build --release
```
3. Flash the binary to the microcontroller:
``` sh
cargo run --release
```

## 📁 Project Structure
    .    
    ├── src/
    │   └── main.rs
    ├── Cargo.lock
    ├── Cargo.toml
    ├── embed.toml
    ├── memory.x
    └── README.md

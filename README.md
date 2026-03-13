# Embedded Rust—LED Flashing Message
An embedded Rust learning project interfacing GPIO pins, peripheral timers, and an I2C-accessed accelerometer.

[![Demonstration Video](https://img.youtube.com/vi/ynV_N79aajg/0.jpg)](https://www.youtube.com/shorts/ynV_N79aajg)

## Overview
This is a project to help me learn Embedded Rust using the Micro:Bit v2 development board. This board features an ARM Cortex M-4 based chip (the nrf52833 from Nordic Semiconductor).

## Key Technologies

| Programming Language(s) | Hardware                         | Software/Tooling                                     |
| ----------------------- | -------------------------------- | ---------------------------------------------------- |
| - Rust                  | - Micro:bit v2 development board | - Neovim as IDE </br>- Probe-rs</br>- Cortex M-4 HAL |

## Features / Capabilities
- I2C interface to communicate with the accelerometer 
- A custom ring buffer to act as an integrator to calculate velocity 
- Using GPIO and timer peripherals of the target MCU to light up the LEDs at the correct times
- Separate logic section testable on host architecture
- A surprisingly good arm workout
 
## What I Learned
- How to set up a project with all the tooling necessary for a bare-metal Rust project
- I2C protocol 
- Cortex M-4 based nrf52833 documentation and general function
- Data structure creation (ring buffer) in a bare-metal environment
- Separation of board specific and more general logic into separate crates for better testing
- Dipped my toes into GDB debugging

## Featured Files/Folders

| Folder/File                | Description                                                                                  |
| -------------------------- | -------------------------------------------------------------------------------------------- |
| /src/                      | Contains all the source code for the project                                                 |
| /src/lib.rs                | Separate code broken out into a library crate to facilitate testing on the host architecture |
| /src/main.rs               | The main binary crate for bare-metal flashing                                                |
| /integrator/ring_buffer.rs | Custom ring buffer implementation to be used as an integrator                                |
| /Embed.toml                | Configuration file for Cargo Embed from Probe-rs                                             |

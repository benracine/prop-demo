# Flight Software Toy

## Background

This demo uses **TCP** as the communication protocol. This is really just a demo toy as it provides only \~0.1 ms timing accuracy and is not at all suitable for real Flight Software.

For demonstration purposes, this system employs a soft real-time model using Tokio-based networking, rather than targeting low-level embedded systems or RTOS platforms.

In a real flight software system, I would use a hardware-backed RTOS scheduler—such as PREEMPT-RT, VxWorks, or Zephyr—to guarantee deterministic task scheduling.

## Quick Start

- Start the Flight Computer with `cargo run`. This will install all necessary dependencies automatically per Rust / cargo norms.

- Run `./propulsion_tcp_client.py` to open the interactive command interface.
  Refer to `INSTRUCTIONS.md` for sample commands.

- Watch the Flight Computer log:

  - Command acknowledgement.
  - Command execution.

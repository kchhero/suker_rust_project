# servo-sim

A simple Rust crate for simulating and managing servo motors.  
Supports angle <-> pulse width conversion, channel management, and serialization.

## Features

- Define servo specifications (min/max pulse, min/max angle)
- Convert between angle and pulse
- Manage multiple servo channels
- Serialize/deserialize servo system state (JSON)
- Example included with 6-DOF robotic arm (MG996R)

## Example

```rust
use servo_sim::{ServoSystem, ServoChannel, ServoSpec};

fn main() {
    let mut system = ServoSystem::new();

    system.add_channel(ServoChannel::new(0, "base", ServoSpec::mg996r()));
    system.add_channel(ServoChannel::new(1, "shoulder", ServoSpec::mg996r()));

    // Set angle
    if let Some(servo) = system.get_channel_mut(0) {
        servo.set_angle(90.0);
        println!("Base angle = {}, pulse = {}", servo.angle, servo.get_pulse());
    }

    // Set pulse
    if let Some(servo) = system.get_channel_mut(1) {
        servo.set_pulse(350);
        println!("Shoulder pulse=350 -> angle = {}", servo.angle);
    }
}
```

## How to Run the Example

1. Clone the repository:
   ```bash
   git clone https://github.com/kchhero/suker_rust_project.git
   ```
2. Navigate to the project directory:
   ```bash
   cd suker_rust_project/servo-libs-simulator
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the example:
   ```bash
   cargo run --example basic
   ```

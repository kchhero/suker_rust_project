use servo_libs_simulator::{ServoSystem, ServoChannel, ServoSpec};

fn main() {
    let mut system = ServoSystem::new();

    // MG996R 6 channels robot arms example
    system.add_channel(ServoChannel::new(0, "base", ServoSpec::mg996r()));
    system.add_channel(ServoChannel::new(1, "shoulder", ServoSpec::mg996r()));
    system.add_channel(ServoChannel::new(2, "elbow", ServoSpec::mg996r()));
    system.add_channel(ServoChannel::new(3, "wrist_pitch", ServoSpec::mg996r()));
    system.add_channel(ServoChannel::new(4, "wrist_roll", ServoSpec::mg996r()));
    system.add_channel(ServoChannel::new(5, "gripper", ServoSpec::mg996r()));

    // angle control
    if let Some(servo) = system.get_channel_mut(0) {
        servo.set_angle(90.0);
        println!("Base angle = {}, pulse = {}", servo.angle, servo.get_pulse());
    }

    // pulse control
    if let Some(servo) = system.get_channel_mut(1) {
        servo.set_pulse(350);
        println!("Shoulder pulse=350 -> angle = {}", servo.angle);
    }
}

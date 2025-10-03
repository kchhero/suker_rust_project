use servo_libs_simulator::{ServoSystem, ServoChannel, ServoSpec};

fn main() {
        println!("\n");
        println!("--- Servo Controller Basic Example ---");

        // MG996R spec create
        let spec = ServoSpec::mg996r();

        // MG996R 6 channels robot arms example
        let mut servo0 = ServoChannel::new(0, "base", spec.clone());
        let mut servo1 = ServoChannel::new(1, "shoulder", spec.clone());
        let mut servo2 = ServoChannel::new(2, "elbow", spec.clone());
        let mut servo3 = ServoChannel::new(3, "wrist_pitch", spec.clone());
        let mut servo4 = ServoChannel::new(4, "wrist_roll", spec.clone());
        let mut servo5 = ServoChannel::new(5, "gripper", spec.clone());

        // angle init
        servo0.set_angle(90.0);
        servo1.set_angle(90.0);
        servo2.set_angle(90.0);
        servo3.set_angle(90.0);
        servo4.set_angle(90.0);
        servo5.set_angle(90.0);

        println!(
                "{}: angle {:.1}° → pulse {}",
                servo0.name,
                servo0.angle,
                servo0.get_pulse()
        );
        println!(
                "{}: angle {:.1}° → pulse {}",
                servo1.name,
                servo1.angle,
                servo1.get_pulse()
        );
        println!(
                "{}: angle {:.1}° → pulse {}",
                servo2.name,
                servo2.angle,
                servo2.get_pulse()
        );
        println!(
                "{}: angle {:.1}° → pulse {}",
                servo3.name,
                servo3.angle,
                servo3.get_pulse()
        );
        println!(
                "{}: angle {:.1}° → pulse {}",
                servo4.name,
                servo4.angle,
                servo4.get_pulse()
        );
        println!(
                "{}: angle {:.1}° → pulse {}",
                servo5.name,
                servo5.angle,
                servo5.get_pulse()
        );

        // pulse -> angle convert test
        servo0.set_pulse(350);
        println!(
                "{}: pulse {} → angle {:.1}°",
                servo0.name,
                servo0.get_pulse(),
                servo0.angle
        );
        servo1.set_pulse(400);
        println!(
                "{}: pulse {} → angle {:.1}°",
                servo1.name,
                servo1.get_pulse(),
                servo1.angle
        );
        servo2.set_pulse(450);
        println!(
                "{}: pulse {} → angle {:.1}°",
                servo2.name,
                servo2.get_pulse(),
                servo2.angle
        );
        servo3.set_pulse(500);
        println!(
                "{}: pulse {} → angle {:.1}°",
                servo3.name,
                servo3.get_pulse(),
                servo3.angle
        );
        servo4.set_pulse(550);
        println!(
                "{}: pulse {} → angle {:.1}°",
                servo4.name,
                servo4.get_pulse(),
                servo4.angle
        );
        servo5.set_pulse(600);
        println!(
                "{}: pulse {} → angle {:.1}°",
                servo5.name,
                servo5.get_pulse(),
                servo5.angle
        );

        println!("\n");
        // increment / decrement
        servo0.increment(10.0);
        println!("{} incremented → {:.1}°", servo1.name, servo1.angle);

        servo1.decrement(30.0);
        println!("{} decremented → {:.1}°", servo1.name, servo1.angle);

        servo2.increment(20.0);
        println!("{} incremented → {:.1}°", servo2.name, servo2.angle);

        servo3.decrement(40.0);
        println!("{} decremented → {:.1}°", servo2.name, servo2.angle);

        servo4.increment(20.0);
        println!("{} incremented → {:.1}°", servo4.name, servo4.angle);
        
        servo5.decrement(15.0);
        println!("{} decremented → {:.1}°", servo5.name, servo5.angle);

        // ServoSystem Add channels
        let mut system = ServoSystem::new();
        system.add_channel(servo0);
        system.add_channel(servo1);

        println!("\n--- ServoSystem status ---");
        for servo in &system.channels {
                println!(
                "Channel {} ({}): angle {:.1}° → pulse {}",
                servo.id,
                servo.name,
                servo.angle,
                servo.get_pulse()
                );
        }

        // get_channel_mut 예제
        if let Some(shoulder) = system.get_channel_mut(1) {
                shoulder.set_angle(120.0);
                println!(
                "\nafter update {}: angle {:.1}° → pulse {}",
                shoulder.name,
                shoulder.angle,
                shoulder.get_pulse()
                );
        }
}

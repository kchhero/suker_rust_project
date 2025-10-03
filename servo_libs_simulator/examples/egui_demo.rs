use eframe::egui;
use servo_libs_simulator::{ServoSpec, ServoChannel, ServoSystem};

pub struct ServoApp {
    system: ServoSystem,
}

impl Default for ServoApp {
    fn default() -> Self {
        let mut system = ServoSystem::new();
        system.add_channel(ServoChannel::new(0, "Base", ServoSpec::mg996r()));
        system.add_channel(ServoChannel::new(1, "Shoulder", ServoSpec::mg996r()));
        system.add_channel(ServoChannel::new(2, "Elbow", ServoSpec::mg996r()));
        system.add_channel(ServoChannel::new(3, "Wrist UD", ServoSpec::mg996r()));
        system.add_channel(ServoChannel::new(4, "Wrist LR", ServoSpec::mg996r()));
        system.add_channel(ServoChannel::new(5, "Gripper", ServoSpec::mg996r()));
        Self { system }
    }
}

impl eframe::App for ServoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Servo Simulator Demo (egui 0.32.3)");

            for servo in &mut self.system.channels {
                ui.group(|ui| {
                    ui.label(format!("Channel {}: {}", servo.id, servo.name));

                    let mut angle = servo.angle;
                    if ui
                        .add(egui::Slider::new(
                            &mut angle,
                            servo.spec.min_angle..=servo.spec.max_angle,
                        ))
                        .changed()
                    {
                        servo.set_angle(angle);
                    }

                    ui.label(format!(
                        "Angle: {:.1}° → Pulse: {}",
                        servo.angle,
                        servo.get_pulse()
                    ));

                    // === 시각화 ===
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(100.0, 100.0), egui::Sense::hover());

                    let center = rect.center();
                    let radius = 40.0;
                    let angle_rad = servo.angle.to_radians();

                    let end = egui::pos2(
                        center.x + radius * angle_rad.cos(),
                        center.y - radius * angle_rad.sin(),
                    );

                    let line = egui::Shape::line_segment(
                        [center, end],
                        egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                    );

                    ui.painter().add(line);
                    ui.painter()
                        .circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::GRAY));
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Servo Simulator Demo",
        options,
        Box::new(|_cc| Ok(Box::<ServoApp>::default())),
    )
}

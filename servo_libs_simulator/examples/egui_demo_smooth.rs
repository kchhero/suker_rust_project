use eframe::egui;
use servo_libs_simulator::{ServoSpec, ServoChannel};

struct ServoApp {
    servo: ServoChannel,
    target_angle: f32,
    speed: f32,
    last_update: std::time::Instant,
    input_angle: String,
    input_speed: String,
}

impl ServoApp {
    fn new() -> Self {
        Self {
            servo: ServoChannel::new(0, "servo1", ServoSpec::mg996r()),
            target_angle: 90.0,
            speed: 30.0, // deg/sec
            last_update: std::time::Instant::now(),
            input_angle: String::new(),
            input_speed: String::new(),
        }
    }
}

impl eframe::App for ServoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // smmooth move
        let new_angle = ServoChannel::smooth_move(self.servo.angle, self.target_angle, self.speed, delta);
        self.servo.set_angle(new_angle);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Servo Smooth Move Demo");

            ui.label(format!("Current Angle: {:.2}°", self.servo.angle));
            ui.label(format!("Target Angle: {:.2}°", self.target_angle));
            ui.label(format!("Speed: {:.2} deg/s", self.speed));

            ui.horizontal(|ui| {
                ui.label("Target Angle (0 ~ 180): ");
                ui.text_edit_singleline(&mut self.input_angle);
            });

            ui.horizontal(|ui| {
                ui.label("Speed (deg/s): ");
                ui.text_edit_singleline(&mut self.input_speed);
            });

            if ui.button("Set").clicked() {
                if let Ok(angle) = self.input_angle.parse::<f32>() {
                    self.target_angle = angle.clamp(0.0, 180.0);
                }
                if let Ok(speed) = self.input_speed.parse::<f32>() {
                    self.speed = speed.max(1.0); // minimum speed guarantee
                }
                self.input_angle.clear();
                self.input_speed.clear();
            }

            // Visualization
            let (rect, _) = ui.allocate_exact_size(egui::vec2(200.0, 200.0), egui::Sense::hover());
            let center = rect.center();
            let radius = 80.0;

            let angle_rad = self.servo.angle.to_radians();
            let x = center.x + radius * angle_rad.cos();
            let y = center.y - radius * angle_rad.sin();

            let painter = ui.painter();
            painter.circle_stroke(center, radius, (2.0, egui::Color32::LIGHT_BLUE));
            painter.line_segment(
                [center, egui::pos2(x, y)],
                (3.0, egui::Color32::RED),
            );
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Servo Smooth Move Demo",
        options,
        Box::new(|_cc| Ok(Box::new(ServoApp::new()))),
    )
}
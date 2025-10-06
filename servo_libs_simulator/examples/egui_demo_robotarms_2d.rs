use eframe::egui;
use servo_libs_simulator::{Joint, RobotArm};

struct RobotArmApp {
    robot: RobotArm,
}

impl RobotArmApp {
    fn new() -> Self {
        let mut arm = RobotArm::new();
        arm.add_joint(Joint::new("Base", -90.0, 90.0, 60.0));
        arm.add_joint(Joint::new("Shoulder", -45.0, 90.0, 50.0));
        arm.add_joint(Joint::new("Elbow", 0.0, 135.0, 40.0));
        arm.add_joint(Joint::new("Wrist Up/Down", -90.0, 90.0, 30.0));
        arm.add_joint(Joint::new("Wrist Left/Right", -90.0, 90.0, 20.0));
        arm.add_joint(Joint::new("Gripper", -30.0, 30.0, 15.0));

        Self { robot: arm }
    }
}

impl eframe::App for RobotArmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Robot Arm Controls");

            for joint in &mut self.robot.joints {
                ui.add(
                    egui::Slider::new(&mut joint.angle, joint.min_angle..=joint.max_angle)
                        .text(&joint.name),
                );
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(500.0, 500.0), egui::Sense::hover());
            let center = egui::pos2(rect.center().x, rect.bottom() - 20.0);
            let painter = ui.painter();

            let positions = self.robot.calc_positions((center.x, center.y));
            for i in 0..positions.len() - 1 {
                let p1 = egui::pos2(positions[i].0, positions[i].1);
                let p2 = egui::pos2(positions[i + 1].0, positions[i + 1].1);
                painter.line_segment([p1, p2], (3.0, egui::Color32::LIGHT_BLUE));
                painter.circle_filled(p1, 4.0, egui::Color32::WHITE);
            }
            if let Some(end) = positions.last() {
                painter.circle_filled(egui::pos2(end.0, end.1), 6.0, egui::Color32::RED);
            }
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "6-DOF Robot Arm Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(RobotArmApp::new()))),
    )
}

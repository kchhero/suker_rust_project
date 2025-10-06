use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

// =========================================================================
#[derive(Component)]
struct JointId(u8);

#[derive(Component)]
struct JointAngle(f32);

#[derive(Component)]
pub struct TargetAngle(pub f32); 

#[derive(Component)]
pub struct RotationSpeed(pub f32);

#[derive(Component)]
struct JointAxis(Vec3);

// =========================================================================
struct LinkParams {
    pub name: &'static str,
    pub channel_id: u8,
    pub translation_vector: Vec3,
    pub rotation_axis: Vec3,
    pub initial_angle_deg: f32,
    pub min_angle_deg: f32,
    pub max_angle_deg: f32,
    pub speed_deg_per_sec: f32,
}

const LINK_DATA: [LinkParams; 6] = [
    // Joint 1: Base rotation (Y)
    LinkParams { name: "Base", channel_id: 15, translation_vector: Vec3::new(0.0, 0.5, 0.0), 
    rotation_axis: Vec3::Y, initial_angle_deg: 95.0, min_angle_deg: 0.0, max_angle_deg: 180.0, speed_deg_per_sec: 30.0 },
    // Joint 2: Shoulder (Z)
    LinkParams { name: "Shoulder", channel_id: 14, translation_vector: Vec3::new(2.0, 0.0, 0.0), 
    rotation_axis: Vec3::Z, initial_angle_deg: 100.0, min_angle_deg: 0.0, max_angle_deg: 180.0, speed_deg_per_sec: 30.0 },
    // Joint 3: Elbow (Z)
    LinkParams { name: "Elbow", channel_id: 13, translation_vector: Vec3::new(1.5, 0.0, 0.0), 
    rotation_axis: Vec3::Z, initial_angle_deg: 130.0, min_angle_deg: 0.0, max_angle_deg: 118.0, speed_deg_per_sec: 30.0 },
    // Joint 4: Wrist Up/Down (Z)
    LinkParams { name: "WristUpDown", channel_id: 12, translation_vector: Vec3::new(1.0, 0.0, 0.0), 
    rotation_axis: Vec3::Z, initial_angle_deg: 130.0, min_angle_deg: 0.0, max_angle_deg: 180.0, speed_deg_per_sec: 30.0 },
    // Joint 5: Wrist Left/Right (X)
    LinkParams { name: "WristLeftRight", channel_id: 7, translation_vector: Vec3::new(0.5, 0.0, 0.0), 
    rotation_axis: Vec3::X, initial_angle_deg: 90.0, min_angle_deg: 0.0, max_angle_deg: 180.0, speed_deg_per_sec: 30.0 },
    // Joint 6: Gripper (Z)
    LinkParams { name: "Gripper", channel_id: 6, translation_vector: Vec3::new(0.2, 0.0, 0.0), 
    rotation_axis: Vec3::Z, initial_angle_deg: 90.0, min_angle_deg: 0.0, max_angle_deg: 180.0, speed_deg_per_sec: 30.0 },
];

// =========================================================================
// Bevy application setup
// =========================================================================

fn main() {
    App::new()
       .add_plugins(DefaultPlugins)
       .add_plugins(EguiPlugin) // bevy_egui plugin add
       .add_plugins(RobotArmFKPlugin) 
       .add_systems(Startup, setup_scene_environment) // init scene
       .add_systems(Update, egui_control_system)
       .run();
}

// =========================================================================
// Systems
// =========================================================================
fn setup_scene_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- 1. camera and light ---
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(8.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
       ..default()
    });

    // Point Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
           ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
       ..default()
    });

    // **[light improvement]: Directional Light add (whole light)**
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            color: Color::rgb(0.8, 0.8, 0.8), 
            illuminance: 10000.0,
         ..default()
        },
        // light direction above robot arms
        transform: Transform::from_xyz(0.0, 10.0, 0.0)
           .looking_at(Vec3::ZERO, Vec3::Y),
     ..default()
    });

    // base platform (ground)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(4.0, 0.2, 4.0)),
        material: materials.add(Color::DARK_GRAY),
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
       ..default()
    });
}


/// Egui Control System
/// UI sliders are used to directly manipulate the JointAngle component.
fn egui_control_system(
    mut contexts: EguiContexts,
    // lib.rs에서 정의된 TargetAngle에 가변 접근합니다.
    mut joint_query: Query<(&JointId, &mut TargetAngle)>,
) {
    egui::Window::new("Robot Arm Control (Direct Input & Smooth Move)").show(contexts.ctx_mut(), |ui| {
        ui.heading("Joint Angles (Degrees)");
        ui.separator();

        for (joint_id, mut target_angle) in joint_query.iter_mut() {
            let id_index = joint_id.0 as usize - 1;
            
            // TargetAngle (라디안)을 UI 표시를 위해 도(Degree)로 변환합니다.
            let mut angle_degrees = target_angle.0.to_degrees(); 
            
            let data = &LINK_DATA[id_index];

            ui.label(format!("{}: J{} (Channel ID: {})", data.name, joint_id.0, data.channel_id));
            
            // 1. 슬라이더 (스크롤)
            ui.add(egui::Slider::new(
                &mut angle_degrees, 
                data.min_angle_deg..=data.max_angle_deg // 설정된 관절 한계 사용
            ).text("Slider"));

            // 2. 직접 입력 필드 (Direct Input)
            ui.horizontal(|ui| {
                ui.label("Direct Input:");
                // angle_degrees 변수를 사용하여 슬라이더와 직접 입력 필드가 동기화됩니다.
                ui.add(egui::DragValue::new(&mut angle_degrees)
                  .speed(1.0)
                  .clamp_range(data.min_angle_deg..=data.max_angle_deg)
                  .suffix("°"));
            });
            
            // 변경된 각도(도)를 TargetAngle 컴포넌트에 다시 라디안으로 저장합니다.
            // TargetAngle의 변화는 lib.rs의 angle_smoothing_system에 의해 감지됩니다.
            target_angle.0 = angle_degrees.to_radians();
            
            ui.label(format!("Radian: {:.3} | Range: {}° ~ {}°", target_angle.0, data.min_angle_deg, data.max_angle_deg));
            ui.separator();
        }
    });
}

fn calculate_next_angle(current: f32, target: f32, speed: f32, delta: f32) -> f32 {
    // 0.01도 차이 (라디안으로 변환)
    const TARGET_THRESHOLD_RAD: f32 = 0.01_f32.to_radians(); 

    let mut angle_diff = target - current;

    // 1. 최단 경로 회전 (360도 회전 문제 방지)
    if angle_diff > PI {
        angle_diff -= 2.0 * PI;
    } else if angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    // 2. 목표에 근접했는지 확인 (lib.rs의 0.01도 임계값 반영) [1]
    if angle_diff.abs() < TARGET_THRESHOLD_RAD {
        return target;
    }

    // 3. 이동할 최대 스텝 계산 (속도 * 시간)
    let max_step = speed * delta;
    
    // 4. 최대 스텝만큼만 이동하도록 클램프
    let actual_step = angle_diff.clamp(-max_step, max_step);
    
    // 5. 다음 위치 반환
    current + actual_step
}


// =========================================================================
// IV. Bevy Plugin 정의 (크레이트를 구성하는 시스템 묶음)
// =========================================================================

pub struct RobotArmFKPlugin;

impl Plugin for RobotArmFKPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_robot_entities) 
         .add_systems(Update, (
                angle_smoothing_system,   // smooth_move 로직 적용 [1]
                kinematics_update_system, // Transform 업데이트
           ));
    }
}


// =========================================================================
// V. 시스템 구현
// =========================================================================

/// 1. 로봇팔 엔티티를 계층적으로 스폰하고 초기 컴포넌트를 설정합니다.
fn setup_robot_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut parent_entity = commands.spawn(SpatialBundle::default()).id();

    for (i, data) in LINK_DATA.iter().enumerate() {
        let joint_id_num = (i + 1) as u8;
        let is_last_link = i == LINK_DATA.len() - 1;
        
        let link_length = data.translation_vector.length();
        let mesh_size_dim = 0.2;
        let initial_angle_rad = data.initial_angle_deg.to_radians();
        
        // --- A. Joint Entity (회전 담당) 스폰 ---
        let joint_entity_id = commands.spawn((
            JointId(joint_id_num),
            JointAxis(data.rotation_axis),
            JointAngle(initial_angle_rad), 
            TargetAngle(initial_angle_rad), 
            RotationSpeed(data.speed_deg_per_sec.to_radians()), 
            SpatialBundle {
                transform: Transform::from_rotation(Quat::from_axis_angle(data.rotation_axis, initial_angle_rad)),
          ..default()
            },
        )).set_parent(parent_entity).id(); 

        // --- B. Link Geometry (시각화 및 다음 관절 앵커) ---
        let mesh_material = materials.add(match joint_id_num {
            1 => Color::rgb(0.9, 0.1, 0.1), 
            2 => Color::rgb(0.1, 0.9, 0.1), 
            3 => Color::rgb(0.1, 0.1, 0.9), 
            4 => Color::rgb(0.9, 0.9, 0.1), 
            5 => Color::rgb(0.9, 0.1, 0.9), 
            6 => Color::rgb(0.1, 0.9, 0.9), 
            _ => Color::WHITE,
        });

        let geometry_translation = data.translation_vector / 2.0;
        
        let (x, y, z) = (
            if data.translation_vector.x.abs() > 0.0 { link_length } else { mesh_size_dim },
            if data.translation_vector.y.abs() > 0.0 { link_length } else { mesh_size_dim },
            if data.translation_vector.z.abs() > 0.0 { link_length } else { mesh_size_dim },
        );

        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(x, y, z)),
            material: mesh_material.clone(),
            transform: Transform::from_translation(geometry_translation), 
      ..default()
        }).set_parent(joint_entity_id); 

        // 다음 관절의 앵커 지점 
        let link_anchor_id = commands.spawn(SpatialBundle {
            transform: Transform::from_translation(data.translation_vector), 
      ..default()
        }).set_parent(joint_entity_id).id(); 

        parent_entity = link_anchor_id;
        
        // 마지막 관절(집게) 표시
        if is_last_link {
             commands.entity(parent_entity).with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(mesh_size_dim * 2.0, mesh_size_dim, mesh_size_dim * 2.0)),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_translation(Vec3::new(link_length * 0.5, 0.0, 0.0)), 
              ..default()
                });
            });
        }
    }
}


/// 2. 각도 부드럽게 보간 시스템
fn angle_smoothing_system(
    time: Res<Time>,
    mut query: Query<(&mut JointAngle, &TargetAngle, &RotationSpeed)>,
) {
    let delta_time = time.delta_seconds();

    for (mut current_angle, target_angle, speed) in &mut query {
        current_angle.0 = calculate_next_angle(
            current_angle.0, 
            target_angle.0, 
            speed.0, 
            delta_time
        );
    }
}


/// 3. 순기구학 업데이트 시스템
fn kinematics_update_system(
    mut query: Query<(&JointAxis, &JointAngle, &mut Transform)>,
) {
    for (axis, angle, mut transform) in &mut query {
        let rotation_quat = Quat::from_axis_angle(axis.0, angle.0);
        transform.rotation = rotation_quat;
    }
}
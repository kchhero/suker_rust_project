use serde::{Serialize, Deserialize};

/// Servo motor specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServoSpec {
    pub min_pulse: u32,   // e.g. 100 (for 0 degrees)
    pub max_pulse: u32,   // e.g. 600 (for 180 degrees)
    pub min_angle: f32,   // usually 0.0
    pub max_angle: f32,   // usually 180.0
}

impl ServoSpec {
    pub fn mg996r() -> Self {
        Self {
            min_pulse: 100,
            max_pulse: 600,
            min_angle: 0.0,
            max_angle: 180.0,
        }
    }

    /// Convert angle -> pulse
    pub fn angle_to_pulse(&self, angle: f32) -> u32 {
        let clamped = angle.clamp(self.min_angle, self.max_angle);
        let ratio = (clamped - self.min_angle) / (self.max_angle - self.min_angle);
        self.min_pulse + ((self.max_pulse - self.min_pulse) as f32 * ratio) as u32
    }

    /// Convert pulse -> angle
    pub fn pulse_to_angle(&self, pulse: u32) -> f32 {
        let clamped = pulse.clamp(self.min_pulse, self.max_pulse);
        let ratio = (clamped - self.min_pulse) as f32 / (self.max_pulse - self.min_pulse) as f32;
        self.min_angle + (self.max_angle - self.min_angle) * ratio
    }
}

/// Represents a single servo channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServoChannel {
    pub id: usize,
    pub name: String,
    pub angle: f32,
    pub spec: ServoSpec,
}

impl ServoChannel {
    pub fn new(id: usize, name: &str, spec: ServoSpec) -> Self {
        Self {
            id,
            name: name.to_string(),
            angle: 0.0,
            spec,
        }
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle.clamp(self.spec.min_angle, self.spec.max_angle);
    }

    pub fn increment(&mut self, delta: f32) {
        self.set_angle(self.angle + delta);
    }

    pub fn decrement(&mut self, delta: f32) {
        self.set_angle(self.angle - delta);
    }

    pub fn get_pulse(&self) -> u32 {
        self.spec.angle_to_pulse(self.angle)
    }

    pub fn set_pulse(&mut self, pulse: u32) {
        self.angle = self.spec.pulse_to_angle(pulse);
    }

    pub fn smooth_move(current: f32, target: f32, speed: f32, delta: f32) -> f32 {
        let diff = target - current;
        if diff.abs() < 0.01 {
            return target;
        }
        let step = speed * delta;
        if diff > 0.0 {
            (current + step).min(target)
        } else {
            (current - step).max(target)
        }
    }
}

/// System of multiple servo channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServoSystem {
    pub channels: Vec<ServoChannel>,
}

impl ServoSystem {
    pub fn new() -> Self {
        Self { channels: Vec::new() }
    }

    pub fn add_channel(&mut self, channel: ServoChannel) {
        self.channels.push(channel);
    }

    pub fn get_channel_mut(&mut self, id: usize) -> Option<&mut ServoChannel> {
        self.channels.iter_mut().find(|c| c.id == id)
    }
}

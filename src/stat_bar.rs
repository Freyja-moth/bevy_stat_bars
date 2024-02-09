use crate::prelude::*;

#[derive(Default)]
pub enum StatBarDirection {
    #[default]
    Horizontal,
    Vertical,
}
impl StatBarDirection {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical)
    }
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal)
    }
}

#[derive(Component)]
pub struct StatBar {
    active: bool,
    /// How fast the bar moves
    speed: f32,
    /// Which direction that bar will fill in
    direction: StatBarDirection,
    /// A value between 0 and 1 representing how much of the bar should be filled
    value: f32,
}
impl Default for StatBar {
    fn default() -> Self {
        Self {
            active: false,
            speed: 0.2,
            direction: Default::default(),
            value: 0.,
        }
    }
}
impl StatBar {
    pub fn new() -> Self {
        Self {
            active: true,
            ..Default::default()
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn switch_active(&mut self) {
        self.active = !self.active;
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn change_speed(mut self, speed: f32) -> Self {
        self.speed = speed.clamp(0., 1.);
        self
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0., 1.);
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn change_direction(mut self, direction: StatBarDirection) -> Self {
        self.direction = direction;
        self
    }
    pub fn change_horizontal(mut self) -> Self {
        self.direction = StatBarDirection::Horizontal;
        self
    }
    pub fn change_vertical(mut self) -> Self {
        self.direction = StatBarDirection::Vertical;
        self
    }
    pub fn set_direction(&mut self, direction: StatBarDirection) {
        self.direction = direction;
    }
    pub fn set_horizontal(&mut self) {
        self.direction = StatBarDirection::Horizontal
    }
    pub fn set_vertical(&mut self) {
        self.direction = StatBarDirection::Vertical;
    }
    pub fn direction(&self) -> &StatBarDirection {
        &self.direction
    }

    pub fn change_value(mut self, value: f32) -> Self {
        self.value = value.clamp(0., 1.);
        self
    }
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0., 1.);
    }
    pub fn add_value(&mut self, value: f32) {
        self.value += value * self.speed;
    }
    pub fn value(&self) -> f32 {
        self.value
    }
}

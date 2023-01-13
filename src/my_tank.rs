use serde_json::Value;

pub type CCommand = u64;

pub enum CCommands {}

impl CCommands {
    pub const NONE: CCommand = 0b0;
    pub const MOVE_FORWARD: CCommand = 0b1;
    pub const MOVE_BACKWARD: CCommand = 0b1 << 1;
    pub const ROTATE_TANK_CLOCKWISE: CCommand = 0b1 << 2;
    pub const ROTATE_TANK_COUNTER_CLOCKWISE: CCommand = 0b1 << 3;
    pub const FIRE: CCommand = 0b1 << 4;
    pub const ROTATE_GUN_CLOCKWISE: CCommand = 0b1 << 5;
    pub const ROTATE_GUN_COUNTER_CLOCKWISE: CCommand = 0b1 << 6;
    pub const ROTATE_RADAR_CLOCKWISE: CCommand = 0b1 << 7;
    pub const ROTATE_RADAR_COUNTER_CLOCKWISE: CCommand = 0b1 << 8;
    pub const LOCK_GUN: CCommand = 0b1 << 9;
    pub const UNLOCK_GUN: CCommand = 0b1 << 10;
    pub const LOCK_RADAR: CCommand = 0b1 << 11;
    pub const UNLOCK_RADAR: CCommand = 0b1 << 12;
}

pub struct Tank {
    pub commands: Vec<CCommand>,
}

impl Tank {
    pub fn default() -> Tank {
        Tank {
            commands: Vec::new(),
        }
    }

    pub fn run(&self) {
        todo!()
    }

    pub fn on_event(&self, event: &Value) {
        todo!()
    }
}

pub fn create_tank() -> Tank {
    Tank::default()
}
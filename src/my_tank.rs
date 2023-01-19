use ct_api::{Tank, TankTraits};
use serde_json::Value;

pub struct MyTank {}

impl TankTraits for MyTank {
    fn run(&self) {
        todo!()
    }

    fn on_event(&self, event: &Value) {
        todo!()
    }
}

pub fn create_tank() -> Tank<'static> {
    Tank {
        commands: Vec::new(),
        my_tank:  &MyTank {}
    }
}

use ct_api::*;
use serde_json::Value;

pub struct MyTank {}

impl Tank for MyTank {
    fn run(&mut self, commands: &mut Vec<CCommand>) {
        todo!()
    }

    fn on_event(&mut self, commands: &mut Vec<CCommand>, event: &Value) {
        todo!()
    }
}

pub fn create_tank() -> MyTank {
    MyTank {}
}

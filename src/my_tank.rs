use ct_api::*;
use serde_json::Value;

pub struct MyTank {}

impl TankTraits for MyTank {
    fn run(&self, commands: &mut Vec<CCommand>) {
        println!("run");
        commands.push(CCommands::MOVE_FORWARD);
    }

    fn on_event(&self, commands: &mut Vec<CCommand>, event: &Value) {
        todo!()
    }
}

pub fn create_tank() -> Tank<'static> {
    Tank {
        commands: Vec::new(),
        my_tank:  &MyTank {}
    }
}

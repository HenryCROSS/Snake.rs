use crossterm::event;

use super::{objects::Object_ops, events::Input_Events};

pub enum Actions_State {
    No_Action,
}

pub struct Actions;

impl Actions {
    pub fn do_event(e: &Input_Events, obj: &mut Box<dyn Object_ops>) -> Result<(), String> {
        Ok(obj.do_actions(e)?)
    }
}

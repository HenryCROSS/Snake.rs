use core::fmt;

use crate::libs::objects::*;

pub enum GameObject {
    Snake,
    Food,
}

impl fmt::Display for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameObject::Food => write!(f, "Food"),
            GameObject::Snake => write!(f, "Snake"),
        }
    }
}

impl GameObject {
    pub fn get_symbol(game_object: &GameObject) -> char {
        match game_object {
            GameObject::Food => 'F',
            GameObject::Snake => 'S',
        }
    }
}

pub struct Object_Factory;
impl Object_Factory {
    pub fn new_object(t: &GameObject, x: i32, y: i32, top: i32, bottom: i32, left: i32, right: i32) -> Box<dyn Object_ops> {
        match t {
            GameObject::Food => Box::new(Food::new(x, y, State::Alive, "Food".to_string(), GameObject::get_symbol(&GameObject::Food), top, bottom, left, right)),
            GameObject::Snake => Box::new(Snake::new(x, y, State::Alive, "Snake".to_string(), GameObject::get_symbol(&GameObject::Snake), top, bottom, left, right)),
        }
    }
}

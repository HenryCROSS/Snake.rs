use crate::libs::objects::*;

pub enum GameObject {
    Snake,
    Food,
}

pub struct Object_Factory;
impl Object_Factory {
    pub fn new_object(t: &GameObject) -> Box<dyn Object_ops> {
        match t {
            GameObject::Food => Box::new(Food::new(8, 8, State::Alive, "Food".to_string(), 'F')),
            GameObject::Snake => Box::new(Snake::new(10, 10, State::Alive, "Snake".to_string(), 'S')),
        }
    }
}

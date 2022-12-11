pub mod libs;

use libs::{
    map::{self, Map},
    renderer,
};
use std::thread;
use std::time::Duration;

use crate::libs::*;

fn main() {
    let mut renderer = renderer::Renderer::new();
    let (left, right, top, bottom) = renderer.get_termianl_properties();
    let mut map = Map::new(left as i32, right as i32, top as i32, bottom as i32);

    let mut food = object_factory::Object_Factory::new_object(&object_factory::GameObject::Food);
    let mut snake = object_factory::Object_Factory::new_object(&object_factory::GameObject::Snake);
    map.register(food);
    map.register(snake);
    
    loop {
        map.update();
        renderer.draw(&map);
        // listen
        thread::sleep(Duration::from_millis(1000));
    }
}

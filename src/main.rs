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
    let mut event_handler = events::Event_ops::new(20);

    let food = object_factory::Object_Factory::new_object(&object_factory::GameObject::Food);
    let snake = object_factory::Object_Factory::new_object(&object_factory::GameObject::Snake);

    map.register_event_handle(event_handler);

    map.register_obj(food);
    map.register_obj(snake);
    
    loop {
        map.event_processing();
        if !map.is_running {
            break;
        }
        map.update();
        map.conflict_processing();

        renderer.draw(&map);
        // listen event
        thread::sleep(Duration::from_millis(20));
    }
}

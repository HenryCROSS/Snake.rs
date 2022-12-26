pub mod libs;

use libs::{
    debug::log,
    map::{self, Map},
    object_factory, renderer,
};
use rand::{thread_rng, Rng};

use crate::libs::*;

fn main() {
    log(file!(), line!().to_string().as_str(), "==============");
    log(file!(), line!().to_string().as_str(), "App started");
    let mut renderer = renderer::Renderer::new();
    let (left, right, top, bottom) = renderer.get_termianl_properties();
    let mut map = Map::new(left as i32, right as i32, top as i32, bottom as i32);
    let mut event_handler = events::Event_ops::new(500);
    let mut rng = thread_rng();
    log(
        file!(),
        line!().to_string().as_str(),
        ("left: ".to_string() + left.to_string().as_str()).as_str(),
    );
    log(
        file!(),
        line!().to_string().as_str(),
        ("right: ".to_string() + right.to_string().as_str()).as_str(),
    );
    log(
        file!(),
        line!().to_string().as_str(),
        ("top: ".to_string() + top.to_string().as_str()).as_str(),
    );
    log(
        file!(),
        line!().to_string().as_str(),
        ("bottom: ".to_string() + bottom.to_string().as_str()).as_str(),
    );

    let food = object_factory::Object_Factory::new_object(
        &object_factory::GameObject::Food,
        rng.gen_range(left..right) as i32,
        rng.gen_range(top..bottom) as i32,
        top as i32,
        bottom as i32,
        left as i32,
        right as i32,
    );
    let snake = object_factory::Object_Factory::new_object(
        &object_factory::GameObject::Snake,
        rng.gen_range(left..right) as i32,
        rng.gen_range(top..bottom) as i32,
        top as i32,
        bottom as i32,
        left as i32,
        right as i32,
    );

    map.register_event_handle(event_handler);

    // map.register_obj(food);
    map.register_obj(snake);

    loop {
        map.event_processing();
        log(file!(), line!().to_string().as_str(), "Event processed!");
        if !map.is_running {
            break;
        }
        map.update_map();
        log(file!(), line!().to_string().as_str(), "Updated map");
        map.conflict_processing();
        log(file!(), line!().to_string().as_str(), "Handled conflict");
        map.update_object_by_state();
        map.update_map();
        log(
            file!(),
            line!().to_string().as_str(),
            "Object is updated by state",
        );
        map.update_by_fn(|map| {
            let snake_num = map.get_obj_num(object_factory::GameObject::Snake);
            if snake_num == 0 {
                map.is_running = false;
                return;
            }
            let num = map.get_obj_num(object_factory::GameObject::Food);
            let (top, bottom, left, right) = map.get_map_properties();
            let mut rng = thread_rng();

            if num < 10 {
                let food = object_factory::Object_Factory::new_object(
                    &object_factory::GameObject::Food,
                    rng.gen_range(left..right-1) as i32,
                    rng.gen_range(top..bottom-1) as i32,
                    top,
                    bottom,
                    left,
                    right,
                );

                log(
                    file!(),
                    line!().to_string().as_str(),
                    food.get_state().to_string().as_str()
                );
                map.register_obj(food);
                log(
                    file!(),
                    line!().to_string().as_str(),
                    "Generate Food successed!",
                );
            }
        });

        renderer.draw(&map);

        // listen event
        // thread::sleep(Duration::from_millis(1000));
    }
    log(file!(), line!().to_string().as_str(), "App End!");
}

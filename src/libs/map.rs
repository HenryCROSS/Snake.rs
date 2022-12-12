use crossterm::event::{self, Event};

use super::{
    actions::{self, Actions},
    events::{self, Event_ops, Input_Events},
    objects::Object_ops,
};

pub struct Map {
    // char == u32
    map: Vec<char>,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    objects: Vec<Box<dyn Object_ops>>,
    event_handle: Option<Event_ops>,
    pub is_running: bool,
}

// fn init_map(map: &mut Vec<char>) {
//     map.iter_mut().for_each(|c| *c = ' ');
// }

impl Map {
    pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        let mut map: Vec<char> = Vec::with_capacity((right * bottom) as usize);
        for _ in 0..map.capacity() {
            map.push(' ');
        }
        let objects = Vec::new();

        Self {
            map,
            top,
            bottom,
            left,
            right,
            objects,
            event_handle: None,
            is_running: true,
        }
    }

    pub fn register_obj(&mut self, obj: Box<dyn Object_ops>) {
        self.objects.push(obj);
    }

    pub fn deregister_obj(&mut self, id: u64) {
        self.objects.retain(|o| o.get_id() != id);
    }

    pub fn register_event_handle(&mut self, handle: Event_ops) {
        self.event_handle = Some(handle);
    }

    // update map
    pub fn update(&mut self) {
        self.clear_all();

        for o in &self.objects {
            let xy = o.get_x_y();
            for (x, y) in xy {
                if x < self.left && x > self.right || y < self.top && y > self.bottom {
                    panic!("Object is out of scope");
                }

                self.map[(self.right * y + x) as usize] = o.get_symbol();
            }
        }
    }

    pub fn clear_all(&mut self) {
        for i in 0..self.map.capacity() {
            self.map[i] = ' ';
        }
    }

    pub fn get_map(&self) -> &Vec<char> {
        &self.map
    }

    /**
     * return (top, bottom, left, right)
     */
    pub fn get_map_properties(&self) -> (i32, i32, i32, i32) {
        (self.top, self.bottom, self.left, self.right)
    }

    // handle all objs features
    /**
     * like function requires the objects, and then do the feature
     */
    pub fn conflict_processing(&mut self) {}

    pub fn event_processing(&mut self) {
        if let Some(e) = &mut self.event_handle {
            if let Some(e) = e.get_event_handle() {
                let e = e.next().unwrap();
                if let Input_Events::Input(e) = &e {
                    if let Event::Key(key) = &e {
                        if let event::KeyCode::Esc = key.code {
                            self.is_running = false;
                            return;
                        }
                    }
                }

                // TODO: Add

                // if let Ok(e) = &e.next() {
                //     if let Event::Key(key) = &e {
                //         if let event::KeyCode::Esc = key.code {
                //             self.is_running = false;
                //             return;
                //         }
                //     }
                // }
            }
        }

    }
}

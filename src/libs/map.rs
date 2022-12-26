use crate::log;
use std::{borrow::BorrowMut, collections::HashMap};

use crossterm::event::{self, Event};

use super::{
    actions::{self, Actions},
    events::{self, Event_ops, Input_Events},
    objects::{Object_ops, State}, object_factory::GameObject,
};

pub struct Map {
    // char == u32
    map: Vec<char>,
    conflict_coor_list: Vec<(i32, i32)>,
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
        let conflict_coor_list = Vec::new();

        Self {
            map,
            top,
            bottom,
            left,
            right,
            objects,
            event_handle: None,
            is_running: true,
            conflict_coor_list,
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
    pub fn update_map(&mut self) {
        self.clear_all();
        // let (left, right, top, bottom) = self.get_map_properties();
        // let (top, bottom, left, right) = self.get_map_properties();

        for o in &self.objects {
            let xy = o.get_x_y();
            for (idx, (x, y)) in xy.iter().enumerate() {
                let pos = (self.right * y + x) as usize;

                // store the coordinate if there are more than 1 objs
                if self.map[pos] != ' ' {
                    self.conflict_coor_list.push((*x, *y));
                    log(file!(), line!().to_string().as_str(), "There is conflict");
                }

                if o.get_symbol() == 'S' && idx != 0 {
                    self.map[pos] = '#';
                } else {
                    self.map[pos] = o.get_symbol();
                }
            }
            log(file!(), line!().to_string().as_str(), ("finish update: ".to_string() + o.get_name()).as_str());
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

    pub fn get_obj_num(&self, obj_type: GameObject) -> usize {
        self.objects.iter().filter(|o| o.get_symbol() == GameObject::get_symbol(&obj_type)).count()
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
    pub fn conflict_processing(&mut self) {
        if self.conflict_coor_list.len() == 0 {
            return;
        }

        // let mut objs: HashMap<(i32, i32), Vec<&mut Box<dyn Object_ops>>> = HashMap::new();
        // WHAT? So I store all the objs into a dict
        // for o in &mut self.objects {
        //     objs.entry(o.get_x_y().first().unwrap().clone())
        //         .or_default()
        //         .push(o);
        // }


        for coord in self.conflict_coor_list.pop() {
            // let conflict = objs.get_mut(&coord).unwrap();
            let mut conflict: Vec<_> = self.objects.iter_mut().filter(|o| {
                for (x, y) in o.get_x_y() {
                    if x == coord.0 && y == coord.1{
                        return true
                    }
                };
                false
                // let (x, y)= o.get_x_y().first().unwrap().clone();
                // log(file!(), line!().to_string().as_str(), o.get_symbol().to_string().as_str());
                // log(file!(), line!().to_string().as_str(), ("x: ".to_string()+x.to_string().as_str()).as_str());
                // log(file!(), line!().to_string().as_str(), ("y: ".to_string()+y.to_string().as_str()).as_str());
                // if x == coord.0 && y == coord.1{
                //     true
                // } else {
                //     false
                // }
            }).collect();
            log(file!(), line!().to_string().as_str(), "Conflict Number:!");
            log(file!(), line!().to_string().as_str(), conflict.len().to_string().as_str());

            conflict.sort_by(|a, b| {
                b.get_weights().cmp(&a.get_weights())
            });

            for idx in 0..conflict.len(){
                for be_react in 0..conflict.len(){
                    log(file!(), line!().to_string().as_str(), "Conflict!");
                    if idx == be_react && conflict.len() > 1 {
                        log(file!(), line!().to_string().as_str(), "FOOD");
                        continue;
                    }

                    //TODO:: use trigger_effect and apply the effect
                    let a = conflict[idx].trigger_effect();
                    // let b = conflict[be_react].trigger_effect();

                    // conflict[idx].apply_effect(b);
                    conflict[be_react].apply_effect(a);
                    log(file!(), line!().to_string().as_str(), "It is handled");
                }
            }
        }
    }

    pub fn event_processing(&mut self) {
        let e = self.event_handle.as_mut().unwrap_or_else(|| {
            log(file!(), line!().to_string().as_str(), "No event handler");
            panic!()
        });
        let e = e.get_event_handle().expect("no event handler");
        let e = e.next().expect("Err happened");

        if let Input_Events::Input(e) = &e {
            if let Event::Key(key) = &e {
                if let event::KeyCode::Esc = key.code {
                    self.is_running = false;
                    return;
                }
            }
        }

        // TODO: Add
        for o in &mut self.objects {
            o.do_actions(&e);
        }
    }

    pub fn update_object_by_state(&mut self) {
        self.objects.retain(|o| {
            if let State::Killed = o.get_state() {
                log(file!(), line!().to_string().as_str(), "It is removed");
                false
            } else {
                true
            }
        })
    }

    pub fn update_by_fn(&mut self, f: fn(&mut Map)) {
        f(self)
    }
}

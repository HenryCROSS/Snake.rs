use std::{collections::VecDeque, sync::Mutex};

use lazy_static::lazy_static;

lazy_static!{
    static ref NUM: Mutex<u64> = Mutex::new(0);
}

pub enum State {
    Alive,
    Killed,
}

struct Object {
    id: u64,
    name: String,
    symbol: char,
    state: State,
}

fn id_generator() -> u64 {
    let mut num = NUM.lock().unwrap();
    let new_id = *num;
    *num += 1;
    return new_id
}

pub trait Object_ops {
    /**
     * return vec<(x, y)>
     */
    fn get_x_y(&self) -> Vec<(i32, i32)>;
    fn set_x_y(&mut self, x: i32, y: i32);

    fn get_symbol(&self) -> char;

    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);

    fn get_state(&self) -> &State;
    fn set_state(&mut self, state: State);

    fn get_id(&self) -> u64;
}

pub struct Snake {
    properties: Object,
    body: VecDeque<(i32, i32)>,
}

impl Snake {
    pub fn new(x: i32, y: i32, state: State, name: String, symbol: char) -> Self {
        let properties = Object {
            name,
            symbol,
            state,
            id: id_generator()
        };

        let mut body = VecDeque::new();
        body.push_front((x, y));
        Snake { properties, body }
    }
}

impl Object_ops for Snake {
    fn get_x_y(&self) -> Vec<(i32, i32)> {
        Vec::from_iter(self.body.clone())
    }

    fn get_symbol(&self) -> char {
        self.properties.symbol
    }

    fn set_x_y(&mut self, x: i32, y: i32) {
        self.body.push_front((x, y));
    }

    fn get_name(&self) -> &String {
        &self.properties.name
    }

    fn set_name(&mut self, name: String) {
        self.properties.name = name;
    }

    fn get_state(&self) -> &State {
        &self.properties.state
    }

    fn set_state(&mut self, state: State) {
        self.properties.state = state;
    }

    fn get_id(&self) -> u64 {
        self.properties.id
    }
}

pub struct Food {
    properties: Object,
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(x: i32, y: i32, state: State, name: String, symbol: char) -> Self {
        let properties = Object {
            name,
            symbol,
            state,
            id: id_generator()
        };

        Food { properties, x, y }
    }
}

impl Object_ops for Food {
    // add code here
    fn get_x_y(&self) -> Vec<(i32, i32)> {
        Vec::from_iter([(self.x, self.y)])
    }

    fn get_symbol(&self) -> char {
        self.properties.symbol
    }

    fn set_x_y(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn get_name(&self) -> &String {
        &self.properties.name
    }

    fn set_name(&mut self, name: String) {
        self.properties.name = name;
    }

    fn get_state(&self) -> &State {
        &self.properties.state
    }

    fn set_state(&mut self, state: State) {
        self.properties.state = state;
    }

    fn get_id(&self) -> u64 {
        self.properties.id
    }
}
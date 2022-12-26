use std::{collections::VecDeque, fmt, sync::Mutex};

use crossterm::event;
use lazy_static::lazy_static;

use super::{debug::log, effects::Effect, events::Input_Events};

lazy_static! {
    static ref NUM: Mutex<u64> = Mutex::new(0);
}

pub enum State {
    Alive,
    Killed,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Alive => write!(f, "Alive"),
            State::Killed => write!(f, "Killed"),
        }
    }
}

struct Object {
    id: u64,
    name: String,
    symbol: char,
    state: State,
    weights: i64,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

fn id_generator() -> u64 {
    let mut num = NUM.lock().unwrap();
    let new_id = *num;
    *num += 1;
    return new_id;
}

pub trait Object_ops {
    /**
     * return vec<(x, y)>
     */
    fn get_x_y(&self) -> Vec<(i32, i32)>;
    fn set_x_y(&mut self, x: i32, y: i32);
    fn set_edge(&mut self, top: i32, bottom: i32, left: i32, right: i32);

    fn get_symbol(&self) -> char;

    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);

    fn get_state(&self) -> &State;
    fn set_state(&mut self, state: State);

    fn get_id(&self) -> u64;

    fn get_weights(&self) -> i64;
    fn set_weights(&mut self, weights: i64);

    fn do_actions(&mut self, _e: &Input_Events) -> Result<(), String> {
        Ok(())
    }

    fn trigger_effect(&mut self) -> Effect {
        Effect::None
    }
    fn apply_effect(&mut self, effect: Effect) {}
}

enum Snake_direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct Snake {
    properties: Object,
    direction: Snake_direction,
    body: VecDeque<(i32, i32)>,
}

impl Snake {
    pub fn new(
        x: i32,
        y: i32,
        state: State,
        name: String,
        symbol: char,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
    ) -> Self {
        let properties = Object {
            name,
            symbol,
            state,
            id: id_generator(),
            weights: 10,
            top,
            bottom,
            left,
            right,
        };

        let mut body = VecDeque::new();
        body.push_front((x, y));
        Snake {
            properties,
            body,
            direction: Snake_direction::None,
        }
    }

    pub fn move_by_direction(&mut self) {
        let vec_xy = self.get_x_y();
        let (x, y) = vec_xy.first().unwrap();
        match self.direction {
            Snake_direction::Up => {
                self.set_x_y(*x, (self.properties.right + *y - 1) % self.properties.right)
            }
            Snake_direction::Down => {
                self.set_x_y(*x, (self.properties.right + *y + 1) % self.properties.right)
            }
            Snake_direction::Left => {
                self.set_x_y((self.properties.right + *x - 1) % self.properties.right, *y)
            }
            Snake_direction::Right => {
                self.set_x_y((self.properties.right + *x + 1) % self.properties.right, *y)
            }
            Snake_direction::None => self.set_x_y(*x, *y),
        }
    }
}

impl Object_ops for Snake {
    fn get_x_y(&self) -> Vec<(i32, i32)> {
        Vec::from_iter(self.body.clone())
    }

    fn get_symbol(&self) -> char {
        self.properties.symbol
    }

    // XXX: How to save the pop_back one if it ate the food
    fn set_x_y(&mut self, x: i32, y: i32) {
        self.body.push_front((x, y));
        self.body.pop_back();
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

    fn get_weights(&self) -> i64 {
        self.properties.weights
    }

    fn set_weights(&mut self, weights: i64) {
        self.properties.weights = weights;
    }

    fn do_actions(&mut self, e: &Input_Events) -> Result<(), String> {
        if let Input_Events::Input(e) = e {
            // unimplemented!()
            if let event::Event::Key(key) = e {
                let key = key.code;
                let vec_xy = self.get_x_y();
                let (x, y) = vec_xy[0];

                // wasd for moving
                match key {
                    event::KeyCode::Char('w') => {
                        self.direction = Snake_direction::Up;
                        self.move_by_direction();
                    }
                    event::KeyCode::Char('s') => {
                        self.direction = Snake_direction::Down;
                        self.move_by_direction();
                    }
                    event::KeyCode::Char('a') => {
                        self.direction = Snake_direction::Left;
                        self.move_by_direction();
                    }
                    event::KeyCode::Char('d') => {
                        self.direction = Snake_direction::Right;
                        self.move_by_direction();
                    }
                    _ => {}
                }
            };
        } else {
            // When tick
            self.move_by_direction();
        }

        Ok(())
    }

    fn apply_effect(&mut self, effect: Effect) {
        if let Effect::Eat = effect {
            let vec_xy = self.get_x_y();
            let (x, y) = vec_xy.first().unwrap();
            match self.direction {
                Snake_direction::Up => {
                    self.body.push_back((*x, (self.properties.right + *y + 1) % self.properties.right))
                }
                Snake_direction::Down => {
                    self.body.push_back((*x, (self.properties.right + *y - 1) % self.properties.right))
                }
                Snake_direction::Left => {
                    self.body.push_back(((self.properties.right + *x + 1) % self.properties.right, *y))
                    // self.set_x_y((self.properties.right + *x - 1) % self.properties.right, *y)
                }
                Snake_direction::Right => {
                    self.body.push_back(((self.properties.right + *x - 1) % self.properties.right, *y))
                    // self.set_x_y((self.properties.right + *x + 1) % self.properties.right, *y)
                }
                Snake_direction::None => {},
            }
        }
    }

    fn set_edge(&mut self, top: i32, bottom: i32, left: i32, right: i32) {
        self.properties.top = top;
        self.properties.bottom = bottom;
        self.properties.left = left;
        self.properties.right = right;
    }
}

pub struct Food {
    properties: Object,
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(
        x: i32,
        y: i32,
        state: State,
        name: String,
        symbol: char,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
    ) -> Self {
        let properties = Object {
            name,
            symbol,
            state,
            id: id_generator(),
            weights: 0,
            top,
            bottom,
            left,
            right,
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

    fn get_weights(&self) -> i64 {
        self.properties.weights
    }

    fn set_weights(&mut self, weights: i64) {
        self.properties.weights = weights;
    }

    fn trigger_effect(&mut self) -> Effect {
        if let State::Killed = self.get_state() {
            return Effect::None;
        }

        log(
            file!(),
            line!().to_string().as_str(),
            ">>>>>>>>>>>>>>>>>.trigger FOOD effect",
        );
        self.set_state(State::Killed);
        Effect::Eat
    }

    fn set_edge(&mut self, top: i32, bottom: i32, left: i32, right: i32) {
        self.properties.top = top;
        self.properties.bottom = bottom;
        self.properties.left = left;
        self.properties.right = right;
    }
}

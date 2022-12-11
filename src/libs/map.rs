use super::objects::Object_ops;

pub struct Map {
    // char == u32
    map: Vec<char>,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    objects: Vec<Box<dyn Object_ops>>,
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
        }
    }

    pub fn register(&mut self, obj: Box<dyn Object_ops>) {
        self.objects.push(obj);
    }

    pub fn deregister(&mut self, id: u64) {
        self.objects.retain(|o| o.get_id() != id);
    }

    pub fn update(&mut self) {
        self.clear_all();

        for o in &self.objects {
            let xy = o.get_x_y();
            for (x, y) in xy {
                if x < self.left && x > self.right || y < self.top && y > self.bottom
                {
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
    pub fn get_map_properties(&self) ->  (i32, i32, i32, i32) {
        (self.top, self.bottom, self.left, self.right)
    }
}

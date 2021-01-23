use crate::location::Location;

#[derive(Debug)]
pub struct Node {
    pub loc: Location,
    g: f64,
    f: f64,
    parent: Option<Location>,
    pub visited: bool,
    pub is_wall: bool,
    pub is_path: bool,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Node {
        Node {
            loc: Location{x, y},
            f: f64::INFINITY,
            g: f64::INFINITY,
            parent: None,
            visited: false,
            is_wall: false,
            is_path: false,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    width: i32,
    height: i32,
    nodes: Vec<Node>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut result = Grid { width, height, nodes:vec![] };
        for y in 0..height {
            for x in 0..width {
                result.nodes.push( Node::new(x, y) );
            }
        }
        result
    }

    pub fn randomize_walls(&mut self, percent_chance: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                if rand::random::<u8>() % 100 < percent_chance {
                    let node_index = self.get_node_index(Location{x, y});
                    self.nodes[node_index].is_wall = true;
                }
            }
        }
    }

    pub fn get_node(&self, loc: Location) -> &Node {
        &self.nodes[self.get_node_index(loc)]
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    fn get_node_index(&self, loc: Location) -> usize {
        (loc.y * self.width + loc.x) as usize
    }

    pub fn reset(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let node_index = self.get_node_index(Location{x, y});
                self.nodes[node_index].visited = false;
                self.nodes[node_index].is_path = false;
                self.nodes[node_index].f = f64::INFINITY;
                self.nodes[node_index].g = f64::INFINITY;
                self.nodes[node_index].parent = None;
            }
        }
    }

    fn backtrack_path(&mut self, start: Location, dest: Location) {
        if start != dest {
            let cur_index = self.get_node_index(start);
            self.nodes[cur_index].is_path = true;
            self.backtrack_path(self.nodes[cur_index].parent.unwrap(), dest);
        }
    }

    pub fn a_star(&mut self, start: Location, dest: Location) {
        let mut open_set: Vec<Location> = Vec::new();

        let start_index = self.get_node_index(start);
        self.nodes[start_index].f = 0.0;
        self.nodes[start_index].g = Location::dist(start, dest);
        open_set.push(start);
        let mut cur_loc;
        
        while !open_set.is_empty() {
            open_set.sort_by(|a, b| {
                let a_node_f = &self.nodes[self.get_node_index(*a)].f;
                let b_node_f = &self.nodes[self.get_node_index(*b)].f;
                a_node_f.partial_cmp(b_node_f).unwrap()
            });
            cur_loc = open_set.swap_remove(0);

            if cur_loc == dest {
                self.backtrack_path(dest, start);
                return;
            }

            let mut neighbors = Vec::new();
            if cur_loc.x > 0 {
                neighbors.push(Location{x: cur_loc.x - 1, y: cur_loc.y});
            }
            if cur_loc.y > 0 {
                neighbors.push(Location{x: cur_loc.x, y: cur_loc.y - 1});
            }
            if cur_loc.y < self.height - 1 {
                neighbors.push(Location{x: cur_loc.x, y: cur_loc.y + 1});
            }
            if cur_loc.x < self.width - 1 {
                neighbors.push(Location{x: cur_loc.x + 1, y: cur_loc.y});
            }

            let cur_index = self.get_node_index(cur_loc);
            self.nodes[cur_index].visited = true;

            for neighbor_loc in neighbors {
                let tentative_g = self.nodes[self.get_node_index(cur_loc)].g + Location::dist(cur_loc, neighbor_loc);
                let neighbor_index = self.get_node_index(neighbor_loc);
                if tentative_g < self.nodes[neighbor_index].g {
                    self.nodes[neighbor_index].parent = Some(cur_loc);
                    self.nodes[neighbor_index].g = tentative_g;
                    self.nodes[neighbor_index].f = tentative_g + Location::dist(neighbor_loc, dest);
                    if !open_set.contains(&neighbor_loc)
                    && !self.nodes[neighbor_index].is_wall
                    && !self.nodes[neighbor_index].visited {
                        open_set.push(neighbor_loc);
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        for (pos, node) in self.nodes.iter().enumerate() {
            if node.is_path {
                print!("x");
            } else if node.is_wall {
                print!("-")
            } else {
                print!(".");
            }
            if ((pos + 1) as i32) % self.width == 0 {
                println!(""); // New line
            }
        }
    }
}
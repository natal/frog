// In this example we are trying to solve the following problem:
//  - let n be some positive number.
//  - we have n * n electronical components placed on a square grid.
//  - each component is wired to its neighbors using 4-connexity.
//  - once the wiring has been done, components having randomly displacet on the grid.
// The goal is to find again the original components placements, such minimizing the length of
// every wire.

extern mod nalgebra;
extern mod frog;

use std::vec;
use std::rand::Rng;
use std::rand;
use nalgebra::na::Vec2;
use nalgebra::na;
use frog::simulated_annealing;

fn main() {
    let n           = 5;
    let mut grid    = Grid::new(n, 5);

    println!("Optimal energy: {}", grid.total_wire_length());

    grid.mess_up();

    do simulated_annealing::solve(20000000000, 25.0, 100 * n, 25 * n, 3,
                                  &mut grid, |cs| cs.total_wire_length()) |grid, accept| {
        // Elementary modification: swap two components.

        let mut id1 = 0;
        let mut id2 = 0;

        while id1 == id2 {
            id1 = rand::task_rng().gen_integer_range(0, grid.ncomponents());
            id2 = rand::task_rng().gen_integer_range(0, grid.ncomponents());
        }

        grid.swap_components(id1, id2);

        // If the solution is not accepted by the simulated anealing, revert the modification. 
        if !accept(grid) {
            // revert the modifications
            grid.swap_components(id1, id2);
        }
    }

    println!("Final energy: {:?}", grid.total_wire_length());
}

#[deriving(ToStr)]
struct Grid {
    priv width:          uint,
    priv min_block_dist: uint,
    priv components:     ~[Component],
    priv components_pos: ~[uint]
}

impl Grid {
    /// Creates a new, optimally organized grid.
    pub fn new(width: uint, min_block_dist: uint) -> Grid {
        let components = do vec::from_fn(width * width) |id| {
            let mut neighbors = ~[];

            if id % width != 0 {
                neighbors.push(id - 1);
            }
            if id % width != width - 1 {
                neighbors.push(id + 1);
            }
            if id / width != 0 {
                neighbors.push(id - width);
            }
            if id / width != width - 1 {
                neighbors.push(id + width);
            }

            Component {
                id:        id,
                neighbors: neighbors
            }
        };

        let pos = vec::from_fn(width * width, |i| i);

        Grid {
            width:          width,
            min_block_dist: min_block_dist,
            components:     components,
            components_pos: pos
        }
    }

    pub fn ncomponents(&self) -> uint {
        self.components.len()
    }

    /// Randomly moves components on the grid.
    pub fn mess_up(&mut self) {
        rand::task_rng().shuffle_mut(self.components);

        for (i, c) in self.components.iter().enumerate() {
            self.components_pos[c.id] = i;
        }
    }

    /// total lentgh of wires
    pub fn total_wire_length(&self) -> f64 {
        let mut length = 0.0;

        for c in self.components.iter() {
            for n in c.neighbors.iter() {
                if c.id < *n {
                    // distance between the two components
                    let a: Vec2<f64> = na::cast(self.component_pos(c));
                    let b: Vec2<f64> = na::cast(self.component_pos_from_id(*n));

                    length = length + na::norm(&(a - b));
                }
            }
        }

        length
    }

    /// Swaps two elements from the grid.
    pub fn swap_components(&mut self, id1: uint, id2: uint) {
        self.components.swap(id1, id2);

        self.components_pos[self.components[id1].id] = id1;
        self.components_pos[self.components[id2].id] = id2;
    }

    /// The position of a component.
    pub fn component_pos(&self, c: &Component) -> Vec2<uint> {
        self.component_pos_from_id(c.id)
    }

    fn component_pos_from_id(&self, id: uint) -> Vec2<uint> {
        na::vec2(
            self.components_pos[id] % self.width * self.min_block_dist,
            self.components_pos[id] / self.width * self.min_block_dist)
    }
}

#[deriving(ToStr)]
struct Component {
    id:        uint,
    neighbors: ~[uint]
}

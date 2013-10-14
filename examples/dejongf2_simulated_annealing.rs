extern mod nalgebra;
extern mod extra;
extern mod frog;

use std::rand;
use nalgebra::na::{Vec2, Norm};
use frog::simulated_annealing;

fn main() {
    let mut x: Vec2<f64> = rand::random();
    x = x * 5.0;

    println!("Initial energy: {}", dejongf2(&Vec2::new(1.0, 1.0)));
    println!("Initial energy: {}", dejongf2(&Vec2::new(0.0, 0.0)));

    simulated_annealing::solve(
        200000000000,
        1.0,
        100,
        25,
        3,
        &mut x,
        dejongf2,
        neighbor);

    println!("Solution: {:?}", x);
    println!("Final energy: {}", dejongf2(&x));
}

fn dejongf2(x: &Vec2<f64>) -> f64 {
    100.0 * (x.x * x.x - x.y).pow(&2.0) + (1.0 - x.x).pow(&2.0)
}

fn neighbor(curr: &mut Vec2<f64>, accept: &fn(&Vec2<f64>) -> bool) {
    let mut direction: Vec2<f64> = rand::random();
    direction = direction - 0.5;

    direction.normalize();

    let step: f64 = 1.0;
    let ammount = step * rand::random();
    let next = (direction * ammount) + *curr;

    if accept(&next) {
        *curr = next;
    }
}

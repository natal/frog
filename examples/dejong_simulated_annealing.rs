extern mod nalgebra;
extern mod extra;
extern mod frog;

use std::rand;
use nalgebra::na::{DVec, Norm};
use nalgebra::na;
use frog::simulated_annealing;

fn main() {
    let n = 20; // number of dimensions

    let mut x = (DVec::new_random(n) - 0.5) * 10.0;

    println!("Initial energy: {}", dejong(&x));

    simulated_annealing::solve(
        2000000,
        1.0,
        100,
        25,
        3,
        &mut x,
        dejong,
        neighbor);

    println!("Solution: {:?}", x);
    println!("Final energy: {}", dejong(&x));
}

fn dejong(x: &DVec<f64>) -> f64 {
    na::sqnorm(x)
}

fn neighbor(curr: &mut DVec<f64>, accept: &fn(&DVec<f64>) -> bool) {
    let mut direction: DVec<f64> = DVec::new_random(curr.len()) - 0.5;

    direction.normalize();

    let step: f64 = 1.0;
    let ammount = step * rand::random();
    let next = (direction * ammount) + *curr;

    if accept(&next) {
        *curr = next;
    }
}

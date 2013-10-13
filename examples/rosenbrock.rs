extern mod extra;
extern mod frog;
use std::rand;
use std::vec;
use extra::time;
use frog::ga::GeneticAlgorithm;

fn bench(name: &str, thresh: f64, ga: &mut GeneticAlgorithm<f64>, fit_fn: &fn(code: &[f64]) -> f64)
{
    let old = time::precise_time_s();
    println("Testing " + name +" function\n");
    let mut count = 0u;
    while ga.best.score <= thresh {
      do ga.step |c| {
        fit_fn(c)
      }
      count = count + 1;
      println("Epoch : " + count.to_str() + " - best score: " + (ga.best.score).to_str());
    }

    let rand_code: ~[f64] = vec::from_fn(ga.best.code.len(), |_| rand::random());
    let time = time::precise_time_s() - old;

    println("\nElpased time: " + time.to_str() + " seconds\n");
    println("Best generation : " + ga.best.code.to_str());
    println("with performance : " + ga.best.score.to_str());
    println("\nRand generation : " + rand_code.to_str());
    println("with performance : " + fit_fn(rand_code).to_str());
}


#[main]
fn main() {
    let mut ga: GeneticAlgorithm<f64> = GeneticAlgorithm::new(4000, 10, 0.5, 50.0, 130);
    do bench("Rosenbrock", 9.19, &mut ga) |c| {
        // Restrict to hypercube -5.12 <= x <= 5.12

        let bmax = 2.0 * 2.048;
        let mut res  = 0.0;
        for (i, e) in c.iter().take(c.len() - 1).enumerate() {
            let e1 = (*e - 0.5) * bmax;
            let e2: f64 = (c[i + 1] - 0.5) * bmax;
            res += 100.0 * (e1.pow(~2.0) - e2).pow(~2.0) + (e1 - 1.0);
        }
        -res
    }
}

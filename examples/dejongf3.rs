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
    let mut ga: GeneticAlgorithm<f64> = GeneticAlgorithm::new(3000, 20, 0.8, 60.0, 100);
    let bmax = 5.12 * 2.0;
    do bench("De Jong f3", 9.0, &mut ga) |c| {
        // Restrict to hypercube -5.12 <= x <= 5.12
        0.0 - c.iter().fold(0.0, |a, e| {
                                let e2 = (*e - 0.5) * bmax;
                                a + ((e2 as int) as f64)
                            })
    }
}

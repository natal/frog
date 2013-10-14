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

    let mut test_arr: ~[f64] = ~[];
    test_arr.push(0.5);
    test_arr.push(0.25);
    println(fit_fn(test_arr).to_str());
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
    let mut ga: GeneticAlgorithm<f64> = GeneticAlgorithm::new(3000, 2, 0.8, 60.0, 100);
    let bmax = 2.0 * 2.0;
    do bench("Goldstein", -3.0001, &mut ga) |c| {
        // Restrict to hypercube -5.12 <= x <= 5.12
        let e1 = (c[0] - 0.5) * bmax;
        let e2 = (c[1] - 0.5) * bmax;

        let a = 1.0 + (e1 + e2 + 1.0).pow(~2.0) *
                 (19.0 - 14.0 * e1 + 13.0 * e1.pow(~2.0) - 14.0 * e2 +
                  6.0 * e1 * e2 + 3.0 * e2.pow(~2.0));
        let b =  30.0 + (2.0 * e1 - 3.0 * e2).pow(~2.0) *
                 (18.0 - 32.0 * e1 + 12.0 * e1.pow(~2.0) + 48.0 * e2 -
                  36.0 * e1 * e2 + 27.0 * e2 * e2);
        -a * b
    }
}

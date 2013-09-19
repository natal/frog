extern mod extra;
extern mod frog;
use std::rand;
use std::vec;
use extra::time;
use frog::ga::GeneticAlgorithm;

//#[start]
//fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
//    std::rt::start_on_main_thread(argc, argv, crate_map, main)
//}

fn bench_ofn(name: &str, nb_iter: uint, ga: &mut GeneticAlgorithm<f64>, fit_fn: &fn(code: &[f64]) -> f64)
{
    let old = time::precise_time_s();
    println("Testing " + name +" function");
    let mut count = 0u;
    do nb_iter.times {
      do ga.step |c| {
        fit_fn(c)
      }
      count = count + 1;
      if (nb_iter > 20u && (count % (nb_iter / 20u) == 0)) {
        println(count.to_str() + "/" + nb_iter.to_str());
        println("Best score: " + (-ga.best.score).to_str());
      }
    }

    let rand_code: ~[f64] = vec::from_fn(ga.best.code.len(), |_| rand::random());
    let time = time::precise_time_s() - old;

    println("Elpased time: " + time.to_str() + " seconds");
    println("Best generation : " + ga.best.code.to_str());
    println("with performance : " + ga.best.score.to_str());
    println("Rand generation : " + rand_code.to_str());
    println("with performance : " + fit_fn(rand_code).to_str());
}

#[main]
fn main() {
    //let args : ~[~str] = os::args();
    let mut ga: GeneticAlgorithm<f64> = GeneticAlgorithm::new(8000, 20, 0.2, 50);
    do bench_ofn("De Jong's", 24, &mut ga) |c| {
       0.0 - c.iter().fold(0.0, |a, e2| a + (*e2) * (*e2))
    }
}

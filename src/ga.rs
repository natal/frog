use std::rand::Rand;
use std::rand::Rng;
use std::vec;
use extra::sort;
use extra::bitv::from_fn;
use dna::Dna;

pub struct GeneticAlgorithm<T> {
  population:       ~[Dna<T>],
  mutation_rate:    f64, // ~0.5 (too much or too less leads to bad results
  gene_persistency: f64,
  selection_amount: uint,
  best:             Dna<T>
}

impl<T: Rand + Clone> GeneticAlgorithm<T> {
  pub fn new(pop_size:         uint,
             dna_length:       uint,
             mutation_rate:    f64,
             gene_persistency: f64,
             selection_amount: uint) -> GeneticAlgorithm<T> {

    let population = vec::from_fn(pop_size, |_| Dna::new_random(dna_length));

    GeneticAlgorithm {
      population:       population,
      mutation_rate:    mutation_rate,
      selection_amount: selection_amount,
      gene_persistency: gene_persistency,
      best:             Dna::new_random(dna_length)
    }
  }

  #[inline(always)]
  pub fn step(&mut self,
              fitness_fn: &fn(code: &[T]) -> f64) {

    for d in self.population.mut_iter() {
        d.perturbate(self.mutation_rate, self.gene_persistency);
    }

    for d in self.population.mut_iter() {
      d.score = fitness_fn(d.code);
    }

    sort::quick_sort(self.population, |e1, e2| e1 >= e2);

    if self.population[0] > self.best {
      self.best.code = self.population[0].code.clone();
      self.best.score = self.population[0].score;
    }

    let res_pop =
    {
      let sliced_pop = self.population.slice_to(self.selection_amount);
      do vec::from_fn(self.population.len()) |_| {
        Dna::new_cross_over(sliced_pop)
      }
    };

    self.population = res_pop;
  }
}

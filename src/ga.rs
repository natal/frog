use std::rand;
use std::rand::Rand;
use std::vec;
use extra::sort;
use extra::bitv::from_fn;

#[deriving(ToStr)]
pub struct Dna<T> {
  priv nb_genes:  uint,
  code:           ~[T],
  ages:           ~[uint],
  score:          f64
}

#[deriving(ToStr)]
impl<T: Rand + Clone> Dna<T> {
  pub fn new(nb_genes: uint) -> Dna<T> {
    let max_val: f64 = Bounded::max_value();
    Dna
    {
      nb_genes: nb_genes,
      code:     vec::with_capacity(nb_genes),
      ages:     vec::from_elem(nb_genes, 0u),
      score:    -max_val
    }
  }

  pub fn new_random(nb_genes: uint) -> Dna<T> {
    let code: ~[T] = vec::from_fn(nb_genes, |_| rand::random());
    let max_val: f64 = Bounded::max_value();
    Dna
    {
      nb_genes:  nb_genes,
      code: code,
      ages: vec::from_elem(nb_genes, 0u),
      score: -max_val
    }
  }

  pub fn new_cross_over(parents: &[Dna<T>]) -> Dna<T> {
    let mut code: ~[T] = vec::with_capacity(parents[0].nb_genes);
    let mut ages: ~[uint] = vec::with_capacity(parents[0].nb_genes);

    for i in range(0, parents[0].nb_genes) {
        let rand_val: uint = rand::random();
        let index: uint = rand_val % parents.len();
        ages.push(parents[index].ages[i] + 1u);
        code.push(parents[index].code[i].clone());
    }
    let max_val: f64 = Bounded::max_value();
    Dna
    {
      nb_genes: parents[0].nb_genes,
      code: code,
      ages: ages,
      score: -max_val
    }
  }

  pub fn perturbate(&mut self, mutation_rate: f64, persistency: f64) {

    self.code = do vec::from_fn(self.nb_genes) |i| {
        let rand_val: f64 = rand::random();
        let weight = {
          if persistency <= 0.0 {
            1.0
          }
          else {
            (-(self.ages[i] as f64) / persistency).exp()
          }
        };
        if rand_val <= (mutation_rate * weight) {
          self.ages[i] = 0;
          rand::random()
        }
        else {
          self.code[i].clone()
        }
    };
  }
}

impl<T> Ord for Dna<T>
{
  fn lt(&self, other: &Dna<T>) -> bool
  { self.score < other.score }

  fn le(&self, other: &Dna<T>) -> bool
  { self.score <= other.score }

  fn ge(&self, other: &Dna<T>) -> bool
  { self.score >= other.score }

  fn gt(&self, other: &Dna<T>) -> bool
  { self.score > other.score }
}

impl<T> Eq for Dna<T>
{
  fn eq(&self, other: &Dna<T>) -> bool
  { self.score == other.score }

  fn ne(&self, other: &Dna<T>) -> bool
  { self.score != other.score }
}

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

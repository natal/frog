use std::rand;

/// Solves an optimization problem using the simulated annealing algorithm.
///
/// # Arguments
/// * `niter` - maximum number of times the temperature is to be changed.
/// * `temperature` - The initial temperature.
/// * `max_mut_trials` - Maximum number of mutations attempt before changing the temperature.
/// * `max_mut_accept` - Maximum number of mutations success before changing the temperature.
/// * `max_no_change` - Maximum number of temperature reduction without succeeding mutation before
/// the algorithm stops.
/// * `system` - The problem to solve.
/// * `energy` - A function able to compute an energy from the system. The simulated annealing will
/// try to minimize this energy.
/// * `perturbator` - A function able to change the state of a system. The perturbator itself takes
/// a closure as argument. If this closures returns `false` the perturbator must revert any changes
/// made to the system.
pub fn solve<S>(niter:          uint,
                temperature:    f64,
                max_mut_trials: uint,
                max_mut_accept: uint,
                max_no_change:  uint,
                system:         &mut S,
                energy:         &fn(&S) -> f64,
                perturbator:    &fn(&mut S, &fn(&S) -> bool)) {
    let mut e   = energy(system);
    let mut tmp = temperature;
    let mut nmutations_accepted = 0;
    let mut nmutations          = 0;
    let mut nnochange           = 0;

    for _ in range(0u, niter) {
        do perturbator(system) |system| {
            let new_e     = energy(system);
            let de        = new_e - e;
            let rand: f64 = rand::random();

            nmutations += 1;

            if de < 0.0 || rand <= (-de / tmp).exp() {
                nmutations_accepted += 1;
                e = new_e;

                true
            }
            else {
                false
            }
        }

        if nmutations >= max_mut_trials || nmutations_accepted >= max_mut_accept {
            if nmutations_accepted == 0 {
                nnochange += 1;
            }
            else {
                nnochange = 0;
            }

            nmutations_accepted = 0;
            nmutations = 0;
            // FIXME: let the user choose the score reduction method.
            tmp *= 0.999;

            if nnochange == max_no_change {
                break;
            }
        }
    }
}

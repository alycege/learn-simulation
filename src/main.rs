use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use std::cmp::max;
use structopt::StructOpt;

enum BarrierResult {
    TerminalOptionValue(f64),
    Invalid
}

enum BarrierType {
    In,
    Out
}

enum BarrierDirection{
    Up,
    Down
}

// TODO: update this for double leg
struct BarrierOption {
    strike_price: f64,
    barrier_price: f64,
    // is_single_leg: bool,
    barrier_type: BarrierType,
    barrier_direction: BarrierDirection,
}

impl BarrierOption {
    pub fn initial_state(&self) -> bool{
        match self.barrier_direction {
            In => false,
            Out => true
        }
    }

    pub fn update_state(&self, underlying_price: f64) -> bool {
        match (self.barrier_direction, self.barrier_type) {
            (BarrierDirection::Up, BarrierType::In) => (underlying_price > self.barrier_price),
            (BarrierDirection::Up, BarrierType::Out) => (underlying_price < self.barrier_price),
            (BarrierDirection::Down, BarrierType::In) => (underlying_price < self.barrier_price),
            (BarrierDirection::Down, BarrierType::Out) => (underlying_price > self.barrier_price)
        }
    }
}

fn main() {
    let option_price = simulate_option_price(1000, 100.0, 10, 0.25, 5.0, 140.0, 145.0);
    println!("{:#?}", option_price);
}

fn random_walk(n_steps: usize, mu: f64, sigma_sq: f64) -> std::vec::Vec<f64> {
    let rng = thread_rng();
    let normal_distr = Normal::new(mu, sigma_sq).unwrap();
    let v = normal_distr.sample_iter(rng).take(n_steps).collect();
    return v;
}

fn option_price_terminal(price_vec: std::vec::Vec<f64>, final_price: f64, barrier: BarrierOption) -> BarrierResult {

    let mut state = barrier.initial_state();

    for &p in price_vec.iter() {

        state &= barrier.update_state(p);

        if !state {
            return BarrierResult::Invalid;
        }
    };

    return BarrierResult::TerminalOptionValue((final_price - barrier.strike_price).max(0.0));
}

fn generate_underlying_price(init_price: f64, n_steps: usize, mu: f64, sigma_sq: f64) -> (f64, std::vec::Vec<f64>) {
    let walk_vec = random_walk(n_steps, mu, sigma_sq);
    let mut curr_price = init_price;
    let mut price_vec = Vec::new();

    for &price_delta in walk_vec.iter() {
        curr_price = curr_price + price_delta;
        price_vec.push(curr_price);
    };

    (curr_price, price_vec)
}

fn simulate_option_price(num_iterations: usize, init_price: f64, n_steps: usize, mu: f64, sigma_sq: f64, barrier: BarrierOption) -> f64 {
    let mut results_vec = Vec::new();

    for _i in 0..num_iterations {
        let (underlying_price_expiry, price_vec)  = generate_underlying_price(init_price, n_steps, mu, sigma_sq);

        // Call option
        let price_terminal = option_price_terminal(price_vec, underlying_price_expiry, barrier);

        match price_terminal {
            BarrierResult::TerminalOptionValue(v) => results_vec.push(v),
            BarrierResult::Invalid => results_vec.push(0.0),
        }

        // results_vec.push(price_terminal::TerminalOptionValue as f64);
    };

    let options_sum:f64 = results_vec.iter().sum();
    options_sum / num_iterations as f64
}

use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use std::cmp::max;
use structopt::StructOpt;

enum BarrierResult {
    TerminalOptionValue(f64),
    Invalid
}

struct BarrierOption {
    strike_price: f64,
    option_price: f64,
    barrier_price: f64,
    terminal: Option<BarrierResult>
}

fn main() {
    let option_price = simulate_option_price(1000, 100.0, 10, 0.25, 5.0, 140.0);
    println!("{:#?}", option_price);
}

fn random_walk(n_steps: usize, mu: f64, sigma_sq: f64) -> std::vec::Vec<f64> {
    let rng = thread_rng();
    let normal_distr = Normal::new(mu, sigma_sq).unwrap();
    let v = normal_distr.sample_iter(rng).take(n_steps).collect();

    v
}

// fn terminal_option_price(init_price: f64, n_steps: usize, mu: f64, sigma_sq: f64, strike_price: f64, option_price: f64, barrier_price: f64) -> BarrierOption {
//     let (final_price, price_vec) = generate_underlying_price(init_price, n_steps, mu, sigma_sq);
//     let mut terminal = Option<BarrierResult>;

//     for &p in price_vec.iter() {
//         // out price
//         if p > option.barrier_price {
//             terminal = Some(BarrierResult::Invalid);
//         }
//     };

//     terminal
// }

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

fn simulate_option_price(num_iterations: usize, init_price: f64, n_steps: usize, mu: f64, sigma_sq: f64, strike_price: f64) -> f64 {
    let mut results_vec = Vec::new();

    for _i in 0..num_iterations {
        let final_price  = generate_underlying_price(init_price, n_steps, mu, sigma_sq);

        // Call option
        let option_price = (final_price.0 - strike_price).max(0.0);
        results_vec.push(option_price);
    };

    let options_sum:f64 = results_vec.iter().sum();
    options_sum / num_iterations as f64
}

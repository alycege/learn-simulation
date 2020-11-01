use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use structopt::StructOpt;

fn main() {
    let mut final_val = simulate_price(50, 100.0, 10, 0.5, 1.5);
    println!("{:#?}", final_val);
}

fn random_walk(n_steps: usize, mu: f64, sigma_sq: f64) -> std::vec::Vec<f64> {
    let rng = thread_rng();
    let normal_distr = Normal::new(mu, sigma_sq).unwrap();
    let v = normal_distr.sample_iter(rng).take(n_steps).collect();

    v
}

fn option_price(init_price: f64, strike: f64, option_type: i64) -> f64 {
    // enum for option price or knockout
    //
    let : u64 = match option_type {
    };

}



fn underlying_price(init_price: f64, n_steps: usize, mu: f64, sigma_sq: f64) -> f64 {
    let walk_vec = random_walk(n_steps, mu, sigma_sq);
    let mut curr_price = init_price;


    for &price_delta in walk_vec.iter() {
        curr_price = curr_price + price_delta;
    };

    curr_price
}

fn simulate_price(num_iterations: usize, init_price: f64, n_steps: usize, mu: f64, sigma_sq: f64) -> std::vec::Vec<f64> {
    let mut results_vec = Vec::new();

    for _i in 0..num_iterations {
        let final_price = underlying_price(init_price, n_steps, mu, sigma_sq);
        results_vec.push(final_price);
    }

    results_vec
}

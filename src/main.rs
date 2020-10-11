// use rand::distributions::{Distribution, Normal};
use rand_distr::Normal;
use rand::thread_rng;
use structopt::StructOpt;

fn main() {
    println!("{:#?}", 0);
}

fn random_walk(n_steps: usize, mu: f64, sigma_sq: f64) -> std::vec::Vec<f64> {
    let rng = thread_rng();
    let normal_distr = Normal::new(mu, sigma_sq).unwrap();
    let v: Vec<f64> = normal_distr.sample_iter(rng).take(n_steps).collect();
    v
}



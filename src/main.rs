use rand::distributions::{Distribution, StandardNormal};
use rand::seq::SliceRandom;
use rand::thread_rng;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "secretary", about = "The Secretary Problem.")]
pub struct Opt {
    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "m", long = "num_iterations", default_value = "1000")]
    pub num_iterations: u64,

    #[structopt(short = "n", long = "secretary_pool", default_value = "100")]
    pub n: u64,

    #[structopt(short = "c", long = "cutoff")]
    pub cutoff: Option<f64>,

    #[structopt(short = "q", long = "q_threshold", default_value="90")]
    pub q_threshold: u64,
}

fn main() {
    // println!("Hello, world!");
    // let mut rng = rand::thread_rng();
    // let n1: i8 = rng.gen();
    //
    // println!("Random i8: {}", n1);
    let opt = Opt::from_args();

    // println!("{:#?}", number_generator(5));
    let num_sec = simulate_secretaries(opt.num_iterations, opt.n, opt.cutoff);
    let mut counter = 0;
    for i in num_sec {
        if i > opt.q_threshold {
            counter += 1;
        }
    }
    println!("{:#?}", counter);
}

fn number_generator(n: usize) -> std::vec::Vec<f64> {
    let rng = thread_rng();
    let v: Vec<f64> = StandardNormal.sample_iter(rng).take(n).collect();
    v
}

fn secretary_generator(n: u64, cutoff: Option<f64>) -> (std::vec::Vec<u64>, u64, u64) {
    // Get array of n random integers, shuffled
    let mut rng = thread_rng();
    let mut all_secretaries: Vec<u64> = (1..n + 1).collect();
    all_secretaries.shuffle(&mut rng);

    // Return maximum value of first 1/e of the array
    let cutoff_idx: u64 = match cutoff {
        Some(c) => (c * (n as f64)).floor() as u64,
        None => ((n as f64) / std::f64::consts::E).floor() as u64,
    };
    let max_interviewed: u64 = *all_secretaries
        .iter()
        .take(cutoff_idx as usize)
        .max()
        .unwrap();

    // If a higher value than the first 1/e values of the array exists, return that value,
    // otherwise 0
    let mut hired = 0;
    let remaining_interviews: Vec<u64> = all_secretaries.iter().rev().take((n - cutoff_idx) as usize).map(|x| x.clone()).collect();
    for i in remaining_interviews.iter().rev() {
        if *i > max_interviewed {
            hired = *i;
        }
    };

    // if no secretaries are better than the first interviewed set, then hire the last one interviewed
    if hired == 0 {
        hired = *remaining_interviews.iter().nth(0).unwrap(); //first().unwrap();
    };

    (all_secretaries, max_interviewed, hired)
}

fn simulate_secretaries(num_iterations: u64, n: u64, cutoff: Option<f64>) -> Vec<u64> {
    // Simulate the secretary problem num_iterations times
    let mut results_vec = Vec::new();
    for _i in 0..num_iterations {
        let hired = secretary_generator(n, cutoff);
        results_vec.push(hired.2);
    }

    results_vec
}

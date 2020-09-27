use rand::thread_rng;
use rand::distributions::{Distribution, StandardNormal};
use rand::seq::SliceRandom;

fn main() {
    // println!("Hello, world!");
    // let mut rng = rand::thread_rng();
    // let n1: i8 = rng.gen();
    // 
    // println!("Random i8: {}", n1);

    // println!("{:#?}", number_generator(5));
    println!("{:#?}", secretary_generator(100, None));
}


fn number_generator(n:usize) -> std::vec::Vec<f64> {
    let rng = thread_rng();
    let v: Vec<f64> = StandardNormal.sample_iter(rng).take(n).collect();
    v
}

fn secretary_generator(n:u64, cutoff:Option <f64>) -> (std::vec::Vec<u64>, u64) {
    
    // Get array of n random integers, shuffled 
    let mut rng = thread_rng(); 
    let mut all_secretaries: Vec<u64> = (1..n+1).collect();
    all_secretaries.shuffle(&mut rng);

    // Return maximum value of first 1/e of the array 
    let cutoff_idx: u64 = match cutoff{
        Some (c) => (c*(n as f64)).floor() as u64, 
        None => ((n as f64)/std::f64::consts::E).floor() as u64,
    };
    // let interviewed: Vec<u64> = all_secretaries.iter().take(cutoff_idx as usize).cloned().collect();
    let max_interviewed: u64 = *all_secretaries.iter().take(cutoff_idx as usize).max().unwrap();
    //(all_secretaries, *interviewed.iter().max().unwrap())
    (all_secretaries, max_interviewed)
}

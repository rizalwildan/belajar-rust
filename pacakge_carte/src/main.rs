use rand::Rng;

fn generate_random_numbers() -> i32 {
    rand::thread_rng().gen_range(0..100)
}

fn main() {
    for i in 0..5 {
        println!("random numbers {}: {}", i, generate_random_numbers())
    }
}

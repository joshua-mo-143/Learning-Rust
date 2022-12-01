use rand::{Rng, distributions::{Distribution, Uniform}, rngs::ThreadRng};
use std::io;
fn main() {
    let mut rng = rand::thread_rng();
    coinflip(&mut rng);
}

fn dice(rng: &mut ThreadRng) {
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn generate_nums(rng: &mut ThreadRng) {
    let n1: u8 = rng.gen();
    let n2 : u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Integer between 0 and 10: {}", rng.gen_range(0..10));
    println!("Float between 0 and 10: {}", rng.gen_range(0.0..10.0));
}

fn coinflip(rng: &mut ThreadRng) {
    loop {
        let result = if rng.gen_range(0..2) == 1 {"heads"} else {"tails"};

        println!("You got {}!", result);
    }

}
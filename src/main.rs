// we have to import the modules crate
mod modules;

// we can now use the stuff inside the crate
use crate::modules::time::Clock;
use crate::modules::utils::*;

fn main() {
    let clock = Clock::new(11, 45);
    println!("{}", clock + Clock::new(1, 30));

    let check = check_pari();
    println!("{:?}", check)
}



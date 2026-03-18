use rand::Rng;
use rand::RngExt;
use rand::rng;

fn main() {
    let mut rng = rng();
    let bullet: i32 = rng.random_range(1..=5);

    println!("{}", bullet);

    for i in 1..=5 {
        println!("Pull the trigger");

        if bullet == i {
            println!("💀 Dead");
            break;
        } else {
            println!("😌 Safe");
        }
    }
}
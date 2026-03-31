use rand::RngExt;
use rand::rng;
use std::io;

fn main() {
    let mut rng = rng();
    let bullet: i32 = rng.random_range(1..=6);
    println!("{}", bullet);

    for i in 1..=5 {
        println!("🔫");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if bullet == i {
            death_animation();
            return;
        }
    }
    println!("Allah is with you");
}

fn death_animation() {
    println!(r#"
    ☠️  YOU DIED ☠️
    
    ──────▄▀▄─────▄▀▄
    ─────▄█░░▀▀▀▀▀░░█▄
    ─▄▄──█░░░░░░░░░░░█──▄▄
    █▄▄█─█░░▀░░┬░░▀░░█─█▄▄█
    "#
    );
}

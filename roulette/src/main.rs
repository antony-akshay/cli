use rand;

fn main() {
    let bullet:f32 = rand::random_range(1.0..6.0);
    println!("{}",bullet.floor());
    
}

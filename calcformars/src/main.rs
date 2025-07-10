fn main() {
    println!("Hello, world!");
}

fn calculate_weight_on_mars(earth_weight: f64) -> f64 {
    // Mars has about 0.38 times the gravity of Earth
    let mars_gravity = 0.38;
    earth_weight * mars_gravity
} 

fn main() {
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1.5; // Adjusting for a hypothetical factor
    println!("Weight on Mars: {} kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f64) -> f64 {
    (weight / 9.81) * 3.711
} 

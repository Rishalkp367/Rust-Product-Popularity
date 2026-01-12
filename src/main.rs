// Product Popularity
// Determine if the popularity is fluctuating, increasing, or decreasing.
// Hashmaps, Loops, conditional if

fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len() - 1 {
        if scores[i] < scores[i + 1] {
            decreasing = false;
        } else if scores[i] > scores[i + 1] {
            increasing = false;
        }
    }

    return increasing || decreasing;
}
use std::collections::HashMap;
fn main() {
    let mut product = HashMap::new();

    product.insert("ProductA", vec![1, 2, 3, 4, 5]);
    product.insert("ProductB", vec![5, 4, 3, 2, 1]);
    product.insert("ProductC", vec![1, 3, 2, 4, 5]);

    for (product_id, popularity) in product {
        if popularity_analysis(popularity) {
            println!("{}: Popularity is increasing or decreasing", product_id);
        } else {
            println!("{}: Popularity is fluctuating", product_id);
        }
    }
}

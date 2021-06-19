mod boxplot;

use boxplot::Boxplot;

pub fn main() {
    let votes: Vec<u64> = [1, 2, 3, 4, 5, 4, 7, 56, 12, 150].to_vec();
    let boxplot = Boxplot::new(votes);
    println!("min: {}", boxplot.min);
    println!("max: {}", boxplot.max);
    println!("mean: {}", boxplot.mean);
    println!("median: {}", boxplot.median);
    println!("q1: {}", boxplot.q1);
    println!("q3: {}", boxplot.q3);
}
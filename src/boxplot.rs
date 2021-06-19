pub struct Boxplot {
    pub data: Vec<u64>,
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub median: f64,
    pub q1: f64,
    pub q3: f64,
}

impl Boxplot {
    pub fn new(data: Vec<u64>) -> Boxplot {
        if data.len() == 0 {
            panic!("Cannot create a boxplot with empty data!");
        }
               
        Boxplot {
            min: min(&data),
            max: max(&data),
            mean: mean(&data),
            median: median(&data),
            q1: q1(&data),
            q3: q3(&data),
            data: data,
        }
    }
}

fn min(v: &Vec<u64>) -> u64 {
    v.iter().min().unwrap().clone()
}

fn max(v: &Vec<u64>) -> u64 {
    v.iter().max().unwrap().clone()
}

fn mean(v: &Vec<u64>) -> f64 {
    let sum: u64 = v.iter().sum();
    let n = v.len();
    sum as f64 / n as f64
}

fn median(v: &Vec<u64>) -> f64 {
    let mut y: Vec<u64> = Vec::new();
    for each in v {
        y.push(*each);
    }
    y.sort();

    let n = y.len();
    if n % 2 == 0  {
        (y[(n/2) - 1] + y[n/2]) as f64 / 2.0
    } else {
        y[n/2] as f64
    }
} 

fn q1(v: &Vec<u64>) -> f64 {
    let n = v.len() as f64;  
    let q1 = (n * 0.25).round() as usize;
    (v[q1]) as f64
}

fn q3(v: &Vec<u64>) -> f64 {
    let n = v.len() as f64;  
    let q1 = (n * 0.75).round() as usize;
    (v[q1]) as f64
}


/// TESTS

/// min tests

#[test]
fn test_min_happy_day() {
    let data: Vec<u64> = [10, 2, 5, 9, 12].to_vec();
    assert_eq!(min(&data), 2);
}

#[test]
fn test_min_equal_values() {
    let data: Vec<u64> = [7, 7, 7, 7, 7].to_vec();
    assert_eq!(min(&data), 7);
}

#[test]
fn test_min_one_value() {
    let data: Vec<u64> = [4].to_vec();
    assert_eq!(min(&data), 4);
}

/// max tests

#[test]
fn test_max_happy_day() {
    let data: Vec<u64> = [10, 12, 5, 9, 2].to_vec();
    assert_eq!(max(&data), 12);
}

#[test]
fn test_max_equal_values() {
    let data: Vec<u64> = [7, 7, 7, 7, 7].to_vec();
    assert_eq!(max(&data), 7);
}

#[test]
fn test_max_one_value() {
    let data: Vec<u64> = [4].to_vec();
    assert_eq!(max(&data), 4);
}

/// median

#[test]
fn test_median_happy_day() {
    let data_even: Vec<u64> = [1, 2, 3, 4].to_vec();
    assert_eq!(median(&data_even), 2.5);

    let data_odd: Vec<u64> = [1, 2, 3, 4, 5].to_vec();
    assert_eq!(median(&data_odd), 3.0);
}

#[test]
fn test_median_unordered() {
    let data_even: Vec<u64> = [4, 1, 2, 3].to_vec();
    assert_eq!(median(&data_even), 2.5);

    let data_odd: Vec<u64> = [4, 1, 5, 2, 3].to_vec();
    assert_eq!(median(&data_odd), 3.0);
}
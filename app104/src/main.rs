use std::collections::HashMap;

// Given a slice of i32 numbers,
// find two numbers such that they add up to a specific target number.
fn two_sum(a: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut num_index = HashMap::new();

    for (i, n) in a.iter().enumerate() {
        let complement = target - n;
        if let Some(&j) = num_index.get(&complement) {
            return Some((a[j], *n));
        }
        num_index.insert(n, i);
    }

    None
}

fn main() {
    let a = vec![2, 7, 11, 15];
    let target = 9;

    if let Some((num1, num2)) = two_sum(&a, target) {
        println!(
            "The two numbers that add up to {} are {} and {}",
            target, num1, num2
        );
    } else {
        println!("No such pair found.");
    }
}

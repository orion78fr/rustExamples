use Parity::{Odd, Even};
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    // Mean 29.5
    // Mode 5
    // Median 10 (5 + 15 / 2)
    // let integer_list = [1, 5, 90, 20, 5, 5, 15, 62, 90, 2];

    // Mean 21
    // Mode 5
    // Median 12
    let integer_list = [1, 5, 90, 20, 5, 12, 14];

    if integer_list.len() == 0 {
        panic!("No Data!");
    }

    // Yeah, I know this is stupid and can use vec! macro
    let mut vector = Vec::new();
    for i in integer_list.iter() {
        vector.push(i);
    }
    println!("Vector : {:?}", vector);

    let mut sum = 0;
    for i in &vector {
        sum += **i;
    }
    let mean = sum as f32 / vector.len() as f32;
    println!("The mean is {}", mean);

    vector.sort();
    println!("Sorted vector : {:?}", vector);

    let median = match parity(vector.len() as i32) {
        Odd => *vector[vector.len() / 2] as f32,
        Even => (*vector[vector.len() / 2 - 1] + *vector[vector.len() / 2]) as f32 / 2 as f32
    };
    println!("The median is {}", median);

    let mut occurrences = HashMap::new();
    for i in &vector {
        let count = occurrences.entry(**i).or_insert(0);
        *count += 1;
    }

    let max_entry = occurrences.iter()
        .max_by(|a, b| -> Ordering { a.1.cmp(b.1) })
        .expect("Cannot find the max");
    println!("The mode is {} with {} occurences", max_entry.0, max_entry.1);
}

fn parity(i: i32) -> Parity {
    if i % 2 == 0 {
        return Even;
    } else {
        return Odd;
    }
}

enum Parity {
    Odd,
    Even,
}
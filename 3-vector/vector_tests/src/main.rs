use Parity::{Odd, Even};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    // Mean 29.5
    // Mode 5
    // Median 10 (5 + 15 / 2)
    // let integer_list = [1, 5, 90, 20, 5, 5, 15, 62, 90, 2];

    // Mean 21
    // Mode 5
    // Median 12
    // let integer_list = [1, 5, 90, 20, 5, 12, 14];

    print!("Give a list of integer separated by spaces : ");
    io::stdout().flush().expect("Exception while flushing standard output");
    let mut integer_list_str = String::new();
    io::stdin().read_line(&mut integer_list_str).expect("Error while getting the integer line");

    let mut vector = Vec::new();

    let integer_list = integer_list_str.split_whitespace();
    integer_list.map(|x| -> i32 { x.parse().expect("Cannot parse as an integer") })
        .for_each(|x| -> () {
            vector.push(x);
        });

    if vector.len() == 0 {
        panic!("The vector is empty");
    }
    println!("Vector : {:?}", vector);

    let mut sum = 0;
    for i in &vector {
        sum += *i;
    }
    let mean = sum as f32 / vector.len() as f32;
    println!("The mean is {}", mean);

    vector.sort();
    println!("Sorted vector : {:?}", vector);

    let median = match parity(vector.len() as i32) {
        Odd => vector[vector.len() / 2] as f32,
        Even => (vector[vector.len() / 2 - 1] + vector[vector.len() / 2]) as f32 / 2 as f32
    };
    println!("The median is {}", median);

    let mut occurrences = HashMap::new();
    for i in &vector {
        let count = occurrences.entry(*i).or_insert(0);
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
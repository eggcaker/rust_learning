#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

use std::{iter::Iterator, vec};

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("First element of vec2 is: {}", vec2.len());

    let array1 = [1, 2, 3];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));

    let array2: [i32; 3] = [4, 5, 6];

    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
    println!("First element of array2 is: {}", array2[0]);

    println!("array1 len: {}", array1.len());
    println!("First element of array1 is: {}", array1[0]);
}

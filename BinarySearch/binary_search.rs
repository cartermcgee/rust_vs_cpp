extern crate rand;

use std::time::{Instant};
use rand::Rng;

fn make_vec(size: u32) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();
    for i in 0..size {
        array.push(i as i32);
    }
    return array
}

fn binary_search(array: Vec<i32>, lower: i32, upper: i32, target: i32) -> i32{
    if upper >= lower {
        let mid = (lower + upper) / 2;
        if array[mid as usize] == target { return mid }
        else if array[mid as usize] < target {
            return binary_search(array, mid + 1, upper, target);
        }
        else {
            return binary_search(array, lower, mid - 1, target);
        }
    }
    return -1
}

fn main() {
    let size: i32 = 100000000;
    let array = make_vec((size) as u32);
    let search = rand::thread_rng().gen_range(0, size - 1) ;
    let now = Instant::now();
    let index = binary_search(array, 0, size, search);
    let elapsed = now.elapsed().as_millis();
    println!("Found target at index {}", index);
    println!("\tTook {} ms", elapsed);
}

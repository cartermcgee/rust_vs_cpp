use std::time::{Duration, Instant};

fn run_benchmark(size: u32) -> Duration{
    let mut vec = make_rand_vec(size);
    let now = Instant::now();
    quick_sort(&mut vec);
    return now.elapsed()
}

fn quick_sort(array: &mut [i32]) {
    if !array.is_empty() {
        let partition_index = partition(array);
        let len = array.len();
        quick_sort(&mut array[0..partition_index]);
        quick_sort(&mut array[partition_index + 1..len]);
    }
}

fn partition(array: &mut [i32]) -> usize {
    let len = array.len();
    let pivot = array[len - 1];
    let mut i = 0;
    let mut j = 0;
    while j < len - 1 {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    array.swap(i, len - 1);
    return i
}

fn make_rand_vec(size: u32) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();
    for _ in 0..size {
        array.push(rand::random::<i32>());
    }
    return array
}

fn main() {
    let size = 10000000;
    println!("\nSorting arrays of size {}", size);
    let mut total = 0;
    for i in 0..25{
        let time = run_benchmark(size);
        total += time.as_millis();
        println!("\tSorted array {}, took {}ms", i, time.as_millis());
    }
    println!("\t-----------------------------");
    println!("\tAverage: {}ms", total/25);
}
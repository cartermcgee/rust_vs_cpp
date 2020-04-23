extern crate rand;
use std::time::Instant;
use std::cmp;
use rand::Rng;

fn main() {
    let num_items: i32 = 1000;
    let w = make_rand_vec(num_items);
    let v = make_rand_vec(num_items);
    let n = w.len() - 1;
    let c = num_items;
    let mut memo = vec![vec![-1; w.len() + 1]; (c + 1) as usize];
    let time = Instant::now();
    let max= knapsack(&w, &v, n, c as i32, &mut memo);
    let elapsed = time.elapsed().as_micros() as f64 / 1000.0;
    println!("Max value = {}", max);
    print!("Took {}ms", elapsed);
}

fn knapsack(weights: &Vec<i32>, values: &Vec<i32>, n: usize, capacity: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
    let result: i32;
    if memo[n][capacity as usize] != -1 {
        return memo[n][capacity as usize] as i32;
    }

    if n == 0 || capacity == 0 {
        result = 0;
    }
    else if weights[n] > capacity {
        result = knapsack(weights, values, n-1, capacity, memo);
    }
    else {
        let not_added = knapsack(weights, values, n-1, capacity, memo);
        let added = values[n] + knapsack(weights, values,n-1, capacity - weights[n], memo);
        result = cmp::max(not_added, added);
    }
    memo[n][capacity as usize] = result;
    return result;
}

fn make_rand_vec(size: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut array: Vec<i32> = Vec::new();
    for _ in 0..size {
        array.push(rng.gen_range(0, size) as i32);
    }
    return array
}

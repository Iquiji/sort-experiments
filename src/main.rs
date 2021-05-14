use std::time::Instant;

use rand::{thread_rng, Rng};

fn main() {
    //println!("1/2: {:?}",1u8/2u8);
    let generated_random_arr = generate_array(1000000000);
    // let now = Instant::now();
    // let bubble_sorted = bubble_sort(generated_random_arr.clone());
    // println!("bubble sort took: {:?}",now.elapsed());
    let now = Instant::now();
    let quick_sorted = quick_sort(generated_random_arr.clone());
    println!("quick sort took: {:?}",now.elapsed());
    // println!("before: {:?}, is_sorted: {}",generated_random_arr.clone(),is_sorted(&generated_random_arr));
    // println!("after: {:?}, is_sorted: {}",bubble_sorted,is_sorted(&bubble_sorted));
    // println!("after: {:?}, is_sorted: {}",quick_sorted,is_sorted(&quick_sorted));
}


fn generate_array(len: usize) -> Vec<u8>{
    let mut arr: Vec<u8> = vec![0; len];
    thread_rng().fill(&mut arr[..]);
    arr
}

fn is_sorted(arr: &Vec<u8>) -> bool{
    let mut is_sorted = true;
    let mut previous: &u8 = &0;
    for x in arr{
        if x < &previous {
            is_sorted = false;
        }
        previous = &x;
    }
    is_sorted
}

fn bubble_sort(arr: Vec<u8>)-> Vec<u8>{
    let mut step_counter = 0;
    let mut to_be_sorted = arr;
    while !is_sorted(&to_be_sorted){
        for i in 1..to_be_sorted.len(){
            if to_be_sorted[i] < to_be_sorted[i-1]{
                let temp = to_be_sorted[i];
                to_be_sorted[i] = to_be_sorted[i-1];
                to_be_sorted[i-1] = temp;
            }
            step_counter += 1;
        }
        //println!("this gen: {:?}",&to_be_sorted);
    }
    // println!("bubble sort took: {} steps",step_counter);
    to_be_sorted
}

fn quick_sort(arr: Vec<u8>)-> Vec<u8>{
    if is_sorted(&arr){
        return arr;
    }
    let pivot = arr.len()/2;
    let pivot_value: u8 = arr[pivot];
    // println!("pivot is: {}",pivot);
    let mut part1: Vec<u8> = vec![];
    let mut part2: Vec<u8> = vec![];
    for (iter_x,x) in arr.iter().enumerate(){
        if iter_x == pivot{
            continue;
        }
        if x <= &pivot_value{
            part1.push(*x);
        }else{
            part2.push(*x);
        }
    }
    // println!("before continue: {:?} ||| {:?}",part1,part2);
    part1 = quick_sort(part1);
    part2 = quick_sort(part2);
    // println!("after continue: {:?} ||| {:?}",part1,part2);
    part1.push(pivot_value);
    part1.append(&mut part2);
    part1
}
// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

use rand::Rng;

fn main() {
    let mut list: [i32; 100] = [0; 100];

    // Create a random generator
    let mut rng = rand::thread_rng();

    // Fill the list
    for i in 0..list.len() {
        list[i] = rng.gen_range(1..=50);
    }

    // Iterate over list to fill the Vector
    let mut v: Vec<i32> = Vec::new();
    for n in list {
        v.push(n);
    }
    // List is still usable

    println!("{:?}", list);

    v.sort();
    match v.get(v.len() / 2) {
        Some(n) => println!("The median is {n}."),
        None => println!("The list is empty"),
    }

    let mut map: HashMap<i32, u32> = HashMap::new();
    let mut max = None;

    for n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }
    // v out of scope

    for (key, value) in &map {
        match max {
            Some((_, max_value)) if value > max_value => max = Some((key, value)),
            None => max = Some((key, value)),
            _ => (), // if value <= max_value
        }
    }

    println!("{:?}", map);

    if let Some((key, _)) = max {
        println!("The mode is {}", key)
    }
}

use std::{io, collections::HashMap, cmp::Ordering};

fn main() {
    println!("Input some number of integers, and I will find the mean, median and mode of the set.");

    let mut set_size = String::new();

    println!("First, select the desired size of the set.");
    
    io::stdin()
        .read_line(&mut set_size)
        .expect("Failed to read line");
    
    let set_size: i32 = match set_size.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("I can't coerce that value to an integer. please try again.");
            return
        }
    };

    let mut input_num = 1;
    let mut set: Vec<i32> = Vec::new();

    while input_num <= set_size {
        println!("Enter integer #{}", &input_num);

        let mut int = String::new();

        io::stdin()
            .read_line(&mut int)
            .expect("Failed to read line");
        
        let int: i32 = match int.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I can't coerce that value to an integer. please try again.");
                continue
            }
        };
        set.push(int);
        println!("The set is now {:?}", set);
        input_num += 1;
    }
    

    set.sort_unstable();
    
    println!("The sorted set is {:?}", set);

    let mut sum_of_set: i32 = 0;
    let mut map = HashMap::new();
    println!("The median number in this set is {}", find_median(&set));
    for int in &set {
        sum_of_set += int;
        let count = map.entry(int).or_insert(0);
        *count += 1;
    }

    println!("The mean of this set is {}", sum_of_set / set.len() as i32);

    let mut mode: (&i32, &i32) = (&0, &0);
    for (key, value) in &map {
        match value.cmp(&mode.1) {
            Ordering::Less => {
                continue
            }
            Ordering::Greater => {
                mode.0 = &key;
                mode.1 = &value;
            }
            Ordering::Equal => {
                continue
            }
        }
    }
    let (key, value) = mode;
    println!("At least one of the modes in this set is {key}, which appeared {value} time(s).");
}

fn find_median(set: &Vec<i32>) -> i32 {

    let length = set.len() as i32;
    if length % 2 == 0 {
        return (set[(length / 2) as usize] + set[((length / 2) - 1) as usize]) / 2;
    } else {
        return set[(length / 2) as usize];
    }
    
}
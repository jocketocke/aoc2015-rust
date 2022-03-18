use std::fs;
use std::sync::mpsc;
use std::thread;
extern crate num_cpus;

//fn main() {
//    let path = String::from("input/1day.in");
//    let input = get_input(path);
//    //let result = output_2nd_part(input);
//    let result = output_1st_part(input);
//    println!("{}", result)
//}

fn get_input(path: String) -> String {
    let res = fs::read_to_string(path).expect("Something went wrong reading the file");
    res.to_string()
}

fn output_1st_part(input: String) -> i32 {
    let nbr_cpus = num_cpus::get();
    let len = input.len();
    let (tx, rx) = mpsc::channel();

    let mut vect = Vec::new();
    for i in 0..nbr_cpus {
        let tmp: String = String::from(&input);
        let cl = tx.clone();
        vect.push(thread::spawn(move || {
            let val = count_floor(tmp);
            cl.send(val).unwrap();
        }));
    }

    for i in vect {
        i.join().unwrap();
    }
    for i in rx {
        println!("{}", i);
    }
    count_floor(input)
}

fn count_floor(substring: String) -> i32 {
    let mut floor = 0;
    for i in substring.chars() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn output_2nd_part(input: String) -> usize {
    let mut current_floor = 0;
    for (index, c) in input.chars().enumerate() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        }
        if current_floor == -1 {
            return index + 1;
        }
    }
    return 0;
}

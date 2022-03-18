use fancy_regex::Regex;
use std::fs;

fn main() {
    let input = get_input("input/input5.txt");
    let row = input.split("\n");
    let mut vec: Vec<&str> = Vec::new();
    for i in row {
        if is_nice(i) {
            vec.push(i);
        }
    }
    println!("{}", vec.len());
}

fn is_nice(string: &str) -> bool {
    //    let re1 = Regex::new(r"([a-z])\1+").unwrap(); //Letter more than once
    //    let re2 = Regex::new(r"[aeiou]").unwrap(); // search for vowels
    //    let re3 = Regex::new(r"ab|cd|pq|xy").unwrap(); // search for vowels
    //    let res1 = re1.is_match(string).unwrap();
    //    let res2 = re2.find_iter(string).count() >= 3;
    //    let res3 = re3.is_match(string).unwrap();
    //    res1 && res2 && !res3
    let re1 = Regex::new(r"(..).*\1").unwrap();
    let re2 = Regex::new(r"(.).\1").unwrap();
    re1.is_match(string).unwrap() && re2.is_match(string).unwrap()
}

fn get_input(path: &str) -> String {
    let res = fs::read_to_string(path).expect("Something went wrong reading the file");
    res.to_string()
}

use fancy_regex::Regex;
use std::fs;

fn main() {
    let input = get_input("input/input6.txt");
    let mut board = [[0; 1000]; 1000];
    input
        .split('\n')
        .filter(|i| !i.is_empty())
        .for_each(|c| execute_command(c, &mut board));
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            //if board[i][j] == true {
            //    count += 1;
            //}
            count += board[i][j];
        }
    }

    println!("{}", count);
}

fn get_input(path: &str) -> String {
    let res = fs::read_to_string(path).expect("Something went wrong reading the file");
    res.to_string()
}

fn execute_command(command: &str, board: &mut [[usize; 1000]; 1000]) {
    let command = get_command(command);
    match command.operation {
        Operation::On => turn_on(command, board),
        Operation::Off => turn_off(command, board),
        Operation::Toggle => toggle(command, board),
    }
}

fn turn_on(command: Command, board: &mut [[usize; 1000]; 1000]) {
    let (fx, fy) = command.from;
    let (tx, ty) = command.to;

    for i in fx..=tx {
        for j in fy..=ty {
            //board[i][j] = true
            board[i][j] += 1;
        }
    }
}
fn turn_off(command: Command, board: &mut [[usize; 1000]; 1000]) {
    let (fx, fy) = command.from;
    let (tx, ty) = command.to;

    for i in fx..=tx {
        for j in fy..=ty {
            //board[i][j] = false
            if board[i][j] > 0 {
                board[i][j] -= 1;
            }
        }
    }
}
fn toggle(command: Command, board: &mut [[usize; 1000]; 1000]) {
    let (fx, fy) = command.from;
    let (tx, ty) = command.to;

    for i in fx..=tx {
        for j in fy..=ty {
            //let invert = board[i][j];
            //board[i][j] = !invert;
            board[i][j] += 2;
        }
    }
}

fn get_command(command: &str) -> Command {
    let re = Regex::new(r"\d+").unwrap();
    let vec: Vec<&str> = re
        .find_iter(command)
        .map(|i| i.unwrap())
        .map(|i| i.as_str())
        .collect();

    if command.contains("turn on") {
        Command {
            operation: Operation::On,
            from: (vec[0].parse().unwrap(), vec[1].parse().unwrap()),
            to: (vec[2].parse().unwrap(), vec[3].parse().unwrap()),
        }
    } else if command.contains("turn off") {
        Command {
            operation: Operation::Off,
            from: (vec[0].parse().unwrap(), vec[1].parse().unwrap()),
            to: (vec[2].parse().unwrap(), vec[3].parse().unwrap()),
        }
    } else {
        Command {
            operation: Operation::Toggle,
            from: (vec[0].parse().unwrap(), vec[1].parse().unwrap()),
            to: (vec[2].parse().unwrap(), vec[3].parse().unwrap()),
        }
    }
}

enum Operation {
    On,
    Off,
    Toggle,
}

struct Command {
    operation: Operation,
    from: (usize, usize),
    to: (usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let input = get_input("input/input6.txt");
        for command in input.split('\n') {
            if !command.is_empty() {
                let re = Regex::new(r"\d+").unwrap();
                let vec: Vec<&str> = re
                    .find_iter(command)
                    .map(|i| i.unwrap())
                    .map(|i| i.as_str())
                    .collect();
                println!("{:?}", vec);
                assert_eq!(vec.len(), 4);
            }
        }
    }
    #[test]
    fn parser2() {
        let input = get_input("input/input6.txt");
        for command in input.split("\n") {
            if !command.is_empty() {
                let mut vec: Vec<&str> = Vec::new();
                let re = Regex::new(r"\d+").unwrap();
                for i in re.find_iter(command) {
                    vec.push(i.unwrap().as_str());
                }
                assert_eq!(vec.len(), 4);
            }
        }
    }
}

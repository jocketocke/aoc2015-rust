use std::fs;

fn main() {
    let input = get_input("input/input2.txt");

    let mut list: Vec<Present> = Vec::new();
    for i in input.split('\n') {
        if i.contains("x") {
            let met = i.split('x').collect::<Vec<&str>>();
            list.push(Present::new(
                met[0].parse::<i32>().unwrap(),
                met[1].parse::<i32>().unwrap(),
                met[2].parse::<i32>().unwrap(),
            ));
        }
    }

    let mut total = 0;
    for i in list {
        println!("Read new present with {:?}", i);
        total += i.ribbon;
        //for part 1 total += i.total;
        println!("Total: {}", total);
    }
    println!("{}", total);
}

fn get_input(path: &str) -> String {
    let res = fs::read_to_string(path).expect("Something went wrong reading the file");
    res.to_string()
}

#[derive(Debug)]
struct Present {
    length: i32,
    height: i32,
    width: i32,
    slack: i32,
    ribbon: i32,
    total: i32,
}

impl Present {
    fn new(length: i32, height: i32, width: i32) -> Present {
        let slack = Present::get_min(length, height, width);
        let ribbon = Present::get_rib(length, height, width);
        let tmp = Present {
            length: length,
            height: height,
            width: width,
            slack: slack,
            ribbon: ribbon,
            total: 2 * length * width + 2 * width * height + 2 * height * length + slack,
        };
        tmp
    }

    fn get_min(length: i32, height: i32, width: i32) -> i32 {
        let mut arr: [i32; 3] = [length, height, width];
        arr.sort();
        arr[0] * arr[1]
    }
    fn get_rib(length: i32, height: i32, width: i32) -> i32 {
        let mut arr: [i32; 3] = [length, height, width];
        arr.sort();
        arr[0] + arr[0] + arr[1] + arr[1] + length * height * width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addup() {
        let pr1: Present = Present::new(2, 3, 4);
        assert_eq!(pr1.total, 58);
        assert_eq!(Present::new(1, 1, 1).total, 7);
        assert_eq!(Present::new(4, 5, 6).total, 168);
    }

    #[test]
    fn total() {
        let arr: [Present; 3] = [
            Present::new(2, 3, 4),
            Present::new(1, 1, 1),
            Present::new(4, 5, 6),
        ];

        let mut total = 0;
        for i in arr {
            total += i.total;
        }
        assert_eq!(total, 233);
    }
}

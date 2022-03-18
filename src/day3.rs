use std::fs;

fn main() {
    let path = String::from("input/input3.txt");
    let input = fs::read_to_string(path).expect("Could not read from path");

    let mut map = Lookup::new();
    let mut santa = Position { x: 0, y: 0 };
    let mut robo_santa = Position { x: 0, y: 0 };
    map.increment(Position { x: 0, y: 0 });
    map.increment(Position { x: 0, y: 0 });
    for (i, p) in input.chars().enumerate() {
        //for first part, only use santa
        if i % 2 == 0 {
            match p {
                '^' => santa.y += 1,
                '<' => santa.x -= 1,
                '>' => santa.x += 1,
                'v' => santa.y -= 1,
                _ => break,
            }
            map.increment(Position {
                x: santa.x,
                y: santa.y,
            });
        } else {
            match p {
                '^' => robo_santa.y += 1,
                '<' => robo_santa.x -= 1,
                '>' => robo_santa.x += 1,
                'v' => robo_santa.y -= 1,
                _ => break,
            }
            map.increment(Position {
                x: robo_santa.x,
                y: robo_santa.y,
            });
        }
    }
    let mut vec: Vec<House> = Vec::new();
    for (_, house) in map.map {
        if house.gifts > 0 {
            vec.push(house);
        }
    }

    for h in &vec {
        assert!(h.gifts >= 1)
    }
    println!("{}", vec.len())
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct House {
    gifts: i32,
}

struct Lookup {
    map: std::collections::HashMap<Position, House>,
}

impl House {
    fn new() -> House {
        House { gifts: 1 }
    }
    fn increment(&mut self) -> () {
        self.gifts += 1;
    }
}

impl Lookup {
    fn new() -> Lookup {
        Lookup {
            map: std::collections::HashMap::new(),
        }
    }
    fn increment(&mut self, position: Position) {
        let result = self.map.get_mut(&position);

        match result {
            Some(x) => {
                House::increment(x);
            }
            None => {
                self.map.insert(position, House::new());
            }
        }
    }
}

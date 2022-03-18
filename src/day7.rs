use std::collections::HashMap;
use std::io::BufRead;
use std::num::ParseIntError;
use std::{fs, hash};

fn main() {
    //let x: u16 = 123;
    //let y: u16 = 456;
    //let d = x & y;
    //let e = x | y;
    //let f = x << 2;
    //let g = y >> 2;
    //let h = !x;
    //let i = !y;
    //println!("{}", d); // AND
    //println!("{}", e); // OR
    //println!("{}", f); // LEFT SHIFT
    //println!("{}", g); // RIGHT SHIFT
    //println!("{}", h); // NOT
    //println!("{}", i); // NOT

    let mut map: HashMap<String, u16> = HashMap::new();
    let mut input = get_input("input/input7.txt");
    for _ in 0..200 {
        println!("alive");
        for i in input
            .clone()
            .split('\n')
            .map(|r| r.trim())
            .filter(|r| !r.is_empty())
        {
            println!("{:?}", map.get("a"));
            if i.contains("NOT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[3];
                let val = s[1];
                let n = Not {
                    expression: to_expr(val),
                    dest: dest.to_string(),
                };
                let res = n.calc(&map);
                if res.is_some() {
                    map.insert(n.dest, res.unwrap());
                }
            } else if i.contains("AND") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let a = And {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = a.calc(&map);
                if res.is_some() {
                    map.insert(a.dest, res.unwrap());
                }
            } else if i.contains("OR") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let o = Or {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = o.calc(&map);
                if res.is_some() {
                    map.insert(o.dest, res.unwrap());
                }
            } else if i.contains("LSHIFT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let l = Lshift {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = l.calc(&map);
                if res.is_some() {
                    map.insert(l.dest, res.unwrap());
                }
            } else if i.contains("RSHIFT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let r = Rshift {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = r.calc(&map);
                if res.is_some() {
                    map.insert(r.dest, res.unwrap());
                }
            } else {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[2];
                let val1 = s[0];
                let a = Assign {
                    expression: to_expr(val1),
                    dest: dest.to_string(),
                };
                let res = a.calc(&map);
                if res.is_some() {
                    map.insert(a.dest, res.unwrap());
                }
            }
        }
    }
    let a = map.get("a").unwrap();
    let mut new_map: HashMap<String, u16> = HashMap::new();
    new_map.insert("b".to_string(), a.clone());
    for _ in 0..10000 {
        for i in input
            .clone()
            .split('\n')
            .map(|r| r.trim())
            .filter(|r| !r.is_empty())
        {
            println!("{:?}", new_map.get("a"));
            if i.contains("NOT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[3];
                let val = s[1];
                let n = Not {
                    expression: to_expr(val),
                    dest: dest.to_string(),
                };
                let res = n.calc(&new_map);
                if res.is_some() {
                    if dest != "b" {
                        new_map.insert(n.dest, res.unwrap());
                    }
                }
            } else if i.contains("AND") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let a = And {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = a.calc(&new_map);
                if res.is_some() {
                    if dest != "b" {
                        new_map.insert(a.dest, res.unwrap());
                    }
                }
            } else if i.contains("OR") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let o = Or {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = o.calc(&new_map);
                if res.is_some() {
                    if dest != "b" {
                        new_map.insert(o.dest, res.unwrap());
                    }
                }
            } else if i.contains("LSHIFT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let l = Lshift {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = l.calc(&new_map);
                if res.is_some() {
                    if dest != "b" {
                        new_map.insert(l.dest, res.unwrap());
                    }
                }
            } else if i.contains("RSHIFT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let r = Rshift {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = r.calc(&new_map);
                if res.is_some() {
                    if dest != "b" {
                        new_map.insert(r.dest, res.unwrap());
                    }
                }
            } else {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[2];
                let val1 = s[0];
                let a = Assign {
                    expression: to_expr(val1),
                    dest: dest.to_string(),
                };
                let res = a.calc(&new_map);
                if res.is_some() {
                    if dest != "b" {
                        new_map.insert(a.dest, res.unwrap());
                    }
                }
            }
        }
    }
    let a = new_map.get("a").unwrap();
    println!("{}", a);
}

fn to_expr(expr: &str) -> Expression {
    expr.parse::<u16>()
        .map(|res| Expression::VALUE(res))
        .unwrap_or(Expression::VARIABLE(expr.to_string()))
}

#[derive(Debug, Hash, PartialEq)]
enum Expression {
    VALUE(u16),
    VARIABLE(String),
}

#[derive(Hash, PartialEq)]
struct Assign {
    expression: Expression,
    dest: String,
}
impl Instruction for Assign {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16> {
        Assign::get_expr(&self.expression, map)
    }
    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16> {
        match expr {
            Expression::VALUE(x) => Some(x.clone()),
            Expression::VARIABLE(y) => map.get(y).map(|res| res.clone()),
        }
    }
}
#[derive(Hash, PartialEq)]
struct And {
    expression: [Expression; 2],
    dest: String,
}
impl Instruction for And {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16> {
        let expr1 = And::get_expr(&self.expression[0], map);
        let expr2 = And::get_expr(&self.expression[1], map);
        if expr1.is_some() && expr2.is_some() {
            Some(expr1.unwrap() & expr2.unwrap())
        } else {
            None
        }
    }

    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16> {
        match expr {
            Expression::VALUE(x) => Some(x.clone()),
            Expression::VARIABLE(y) => map.get(y).map(|res| res.clone()),
        }
    }
}
#[derive(Hash, PartialEq)]
struct Or {
    expression: [Expression; 2],
    dest: String,
}
impl Instruction for Or {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16> {
        let expr1 = Or::get_expr(&self.expression[0], map);
        let expr2 = Or::get_expr(&self.expression[1], map);
        if expr1.is_some() && expr2.is_some() {
            Some(expr1.unwrap() | expr2.unwrap())
        } else {
            None
        }
    }

    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16> {
        match expr {
            Expression::VALUE(x) => Some(x.clone()),
            Expression::VARIABLE(y) => map.get(y).map(|res| res.clone()),
        }
    }
}
#[derive(Hash, PartialEq)]
struct Lshift {
    expression: [Expression; 2],
    dest: String,
}
impl Instruction for Lshift {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16> {
        let expr1 = Lshift::get_expr(&self.expression[0], map);
        let expr2 = Lshift::get_expr(&self.expression[1], map);
        if expr1.is_some() && expr2.is_some() {
            Some(expr1.unwrap() << expr2.unwrap())
        } else {
            None
        }
    }

    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16> {
        match expr {
            Expression::VALUE(x) => Some(x.clone()),
            Expression::VARIABLE(y) => map.get(y).map(|res| res.clone()),
        }
    }
}
#[derive(Hash, PartialEq)]
struct Rshift {
    expression: [Expression; 2],
    dest: String,
}
impl Instruction for Rshift {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16> {
        let expr1 = Rshift::get_expr(&self.expression[0], map);
        let expr2 = Rshift::get_expr(&self.expression[1], map);
        if expr1.is_some() && expr2.is_some() {
            Some(expr1.unwrap() >> expr2.unwrap())
        } else {
            None
        }
    }

    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16> {
        match expr {
            Expression::VALUE(x) => Some(x.clone()),
            Expression::VARIABLE(y) => map.get(y).map(|res| res.clone()),
        }
    }
}

#[derive(Hash, PartialEq)]
struct Not {
    expression: Expression,
    dest: String,
}

impl Instruction for Not {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16> {
        let res = Not::get_expr(&self.expression, map);
        res.map(|res| !res)
    }
    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16> {
        match expr {
            Expression::VALUE(x) => Some(x.clone()),
            Expression::VARIABLE(y) => map.get(y).map(|res| res.clone()),
        }
    }
}

trait Instruction {
    fn calc(&self, map: &HashMap<String, u16>) -> Option<u16>;
    fn get_expr(expr: &Expression, map: &HashMap<String, u16>) -> Option<u16>;
}

fn get_input(path: &str) -> String {
    let res = fs::read_to_string(path).expect("Something went wrong reading the file");
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn not_var_fail() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = !15;
        let test: String = "a".to_string();
        map.insert(test, 15);

        let not: Not = Not {
            dest: "tmp".to_string(),
            expression: Expression::VARIABLE("b".to_string()),
        };
        let res = not.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn not_var() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = !15;
        let test: String = "a".to_string();
        map.insert(test, 15);

        let not: Not = Not {
            dest: "tmp".to_string(),
            expression: Expression::VARIABLE("a".to_string()),
        };
        let res = not.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn not() {
        let map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 15;
        let test: u16 = !ans;

        let not: Not = Not {
            dest: "tmp".to_string(),
            expression: Expression::VALUE(test),
        };
        let res = not.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn lshift() {
        let map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 15 << 1;
        let test1: u16 = 15;
        let test2: u16 = 1;

        let lshift: Lshift = Lshift {
            dest: "tmp".to_string(),
            expression: [Expression::VALUE(test1), Expression::VALUE(test2)],
        };
        let res = lshift.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn rshift() {
        let map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 >> 15;
        let test1: u16 = 1;
        let test2: u16 = 15;

        let rshift: Rshift = Rshift {
            dest: "tmp".to_string(),
            expression: [Expression::VALUE(test1), Expression::VALUE(test2)],
        };
        let res = rshift.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn or() {
        let map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 | 15;
        let test1: u16 = 1;
        let test2: u16 = 15;

        let or: Or = Or {
            dest: "tmp".to_string(),
            expression: [Expression::VALUE(test1), Expression::VALUE(test2)],
        };
        let res = or.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn and() {
        let map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 & 15;
        let test1: u16 = 1;
        let test2: u16 = 15;

        let and: And = And {
            dest: "tmp".to_string(),
            expression: [Expression::VALUE(test1), Expression::VALUE(test2)],
        };
        let res = and.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn assign() {
        let map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1;
        let test1: u16 = 1;

        let assign: Assign = Assign {
            dest: "tmp".to_string(),
            expression: Expression::VALUE(test1),
        };
        let res = assign.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn variables_rshift() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 >> 15;
        let test1: u16 = 1;
        let test2: u16 = 15;
        let var = "test";
        map.insert(String::from(var), test2);

        let shift: Rshift = Rshift {
            dest: "tmp".to_string(),
            expression: [
                Expression::VALUE(test1),
                Expression::VARIABLE(var.to_string()),
            ],
        };
        let res = shift.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn variables_lshift() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 << 15;
        let test1: u16 = 1;
        let test2: u16 = 15;
        let var = "test";
        map.insert(String::from(var), test2);

        let shift: Lshift = Lshift {
            dest: "tmp".to_string(),
            expression: [
                Expression::VALUE(test1),
                Expression::VARIABLE(var.to_string()),
            ],
        };
        let res = shift.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn variables_or() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 | 15;
        let test1: u16 = 1;
        let test2: u16 = 15;
        let var = "test";
        map.insert(String::from(var), test2);

        let or: Or = Or {
            dest: "tmp".to_string(),
            expression: [
                Expression::VALUE(test1),
                Expression::VARIABLE(var.to_string()),
            ],
        };
        let res = or.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn variables_and() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 & 15;
        let test1: u16 = 1;
        let test2: u16 = 15;
        let var = "test";
        map.insert(String::from(var), test2);

        let and: And = And {
            dest: "tmp".to_string(),
            expression: [
                Expression::VALUE(test1),
                Expression::VARIABLE(var.to_string()),
            ],
        };
        let res = and.calc(&map);
        assert_eq!(res.unwrap(), ans)
    }
    #[test]
    fn both_variables_and() {
        let mut map: HashMap<String, u16> = HashMap::new();
        let ans: u16 = 1 & 15;
        let test1: u16 = 1;
        let test2: u16 = 15;
        let var1 = "test1";
        let var2 = "test2";
        map.insert(String::from(var1), test1);
        map.insert(String::from(var2), test2);

        let and: And = And {
            dest: "tmp".to_string(),
            expression: [
                Expression::VARIABLE(var1.to_string()),
                Expression::VARIABLE(var2.to_string()),
            ],
        };
        let res = and.calc(&map);
        if res.is_some() {
            map.insert(and.dest, res.unwrap());
        }
        assert_eq!(map.get("tmp").unwrap(), &ans);
    }
    #[test]
    fn example_string() {
        let input = "
                    123 -> x
                    456 -> y
                    x AND y -> d
                    x OR y -> e
                    x LSHIFT 2 -> f
                    y RSHIFT 2 -> g
                    NOT x -> h
                    NOT y -> i
                    "
        .to_string();
        let mut map: HashMap<String, u16> = HashMap::new();
        for i in input
            .split('\n')
            .filter(|r| !r.is_empty())
            .map(|r| r.trim())
            .filter(|r| !r.is_empty())
        {
            if i.contains("NOT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[3];
                let val = s[1];
                let n = Not {
                    expression: to_expr(val),
                    dest: dest.to_string(),
                };
                let res = n.calc(&map);
                if res.is_some() {
                    map.insert(n.dest, res.unwrap());
                }
            } else if i.contains("AND") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let a = And {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = a.calc(&map);
                if res.is_some() {
                    map.insert(a.dest, res.unwrap());
                }
            } else if i.contains("OR") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let o = Or {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = o.calc(&map);
                if res.is_some() {
                    map.insert(o.dest, res.unwrap());
                }
            } else if i.contains("LSHIFT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let l = Lshift {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = l.calc(&map);
                if res.is_some() {
                    map.insert(l.dest, res.unwrap());
                }
            } else if i.contains("RSHIFT") {
                let s: Vec<&str> = i.split(" ").collect();
                let dest = s[4];
                let val1 = s[0];
                let val2 = s[2];
                let r = Rshift {
                    expression: [to_expr(val1), to_expr(val2)],
                    dest: dest.to_string(),
                };
                let res = r.calc(&map);
                if res.is_some() {
                    map.insert(r.dest, res.unwrap());
                }
            } else {
                let s: Vec<&str> = i.split(" ").collect();
                println!("{:?}", s);
                let dest = s[2];
                let val1 = s[0];
                let a = Assign {
                    expression: to_expr(val1),
                    dest: dest.to_string(),
                };
                let res = a.calc(&map);
                if res.is_some() {
                    map.insert(a.dest, res.unwrap());
                }
            }
        }
        println!("{:?}", map);
        println!("hallo");
        let ans = map.get("i").unwrap();
        assert_eq!(ans, &65079);
    }
}

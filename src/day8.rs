use std::fs;
fn main() {
    let input = get_input("input/input8.txt");
    let input_list: Vec<&str> = input.split('\n').collect();
    // Uncomment for solution part 1
    //let ans: usize = input_list
    //    .iter()
    //    .map(|st| st.trim())
    //    .filter(|st| !st.is_empty())
    //    .map(|st| get_code(st) - get_str(st))
    //    .sum();

    let ans: usize = input_list
        .iter()
        .map(|st| st.trim())
        .filter(|st| !st.is_empty())
        .map(|st| get_code(&encode(st)) - get_code(st))
        .sum();

    println!("{}", ans);
}

fn get_input(path: &str) -> String {
    let res = fs::read_to_string(path).expect("Something went wrong reading the file");
    String::from(res)
}

//Gives the code length
fn get_code(string: &str) -> usize {
    string.len()
}

fn encode(string: &str) -> String {
    let mut new_string = String::new();
    new_string.push_str("\"");
    new_string.push_str("\\\"");

    let cut_string: String = string[1..string.len() - 1].to_string();
    for (index, c) in cut_string.chars().enumerate() {
        if c == '\\' {
            let next_c = cut_string.chars().nth(index + 1);
            if next_c.is_none() {
                //meaning it is end of line and case \\
                new_string.push('\\');
                new_string.push('\\');
                break;
            }
            let next_c = next_c.unwrap();
            if next_c == '\"' {
                new_string.push('\\');
                new_string.push('\\');
                new_string.push('\\');
            } else if next_c == '\\' {
                new_string.push('\\');
                new_string.push('\\');
            } else {
                new_string.push('\\');
                new_string.push('\\');
            }
        } else {
            new_string.push(c);
        }
    }

    new_string.push_str("\\\"");
    new_string.push_str("\"");

    new_string
}

//Gives the str length
fn get_str(string: &str) -> usize {
    let cut_string: String = string[1..string.len() - 1].to_string();
    let mut count = 0;
    let mut skip = 0;
    for (index, c) in cut_string.chars().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }
        if c == '\\' {
            let next_char = cut_string.chars().nth(index + 1).unwrap();
            if next_char == '\"' {
                skip += 1;
            } else if next_char == '\\' {
                skip += 1;
            } else {
                skip += 3;
            }
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string() {
        let empty_string = "\"\"";
        assert_eq!(get_str(empty_string), 0);
        assert_eq!(get_code(empty_string), 2);
    }

    #[test]
    fn abc_string() {
        let abc = "\"abc\"";
        assert_eq!(get_str(abc), 3);
        assert_eq!(get_code(abc), 5);
    }

    #[test]
    fn aaa_string() {
        let aaa = "\"aaa\\\"aaa\"";
        assert_eq!(get_str(aaa), 7);
        assert_eq!(get_code(aaa), 10);
    }

    #[test]
    fn x_string() {
        let aaa = "\"\\x27\"";
        assert_eq!(get_str(aaa), 1);
        assert_eq!(get_code(aaa), 6);
    }

    #[test]
    fn add_all() {
        let vec: Vec<&str> = vec!["\"\\x27\"", "\"aaa\\\"aaa\"", "\"abc\"", "\"\""];

        let code_count: usize = vec.iter().map(|st| get_code(st)).sum();
        let str_count: usize = vec.iter().map(|st| get_str(st)).sum();
        assert_eq!(code_count - str_count, 12);
    }

    #[test]
    fn encode_empty() {
        let test = "\"\"";
        let res = encode(test);

        //assert_eq!(res, "\\\"\\\"");
        assert_eq!(get_code(&res), 6);
    }

    #[test]
    fn encode_abc() {
        let test = "\"abc\"";
        let res = encode(test);

        //assert_eq!(res, "\\\"abc\\\"");
        assert_eq!(get_code(&res), 9);
    }

    #[test]
    fn encode_aaa() {
        let test = "\"aaa\\\"aaa\"";
        let res = encode(test);

        //From "aaa\"aaa"
        //To   "\"aaa\\\"aaa\""
        //test "\"aaa\\\"aaa\""
        //res  "\"\\\"aaa\\\\\"aaa\\\"\""

        //assert_eq!(res, "\\\"aaa\\\\\"aaa\\\"");
        assert_eq!(get_code(&res), 16);
    }

    #[test]
    fn encode_x() {
        let test = "\"\\x27\"";
        let res = encode(test);

        //assert_eq!(res, "\\\"\\\\x27\\\"");
        assert_eq!(get_code(&res), 11);
    }

    #[test]
    fn encode_all() {
        let test = ["\"\\x27\"", "\"aaa\\\"aaa\"", "\"abc\"", "\"\""];
        let res_encode: usize = test.iter().map(|st| get_code(&encode(st))).sum();

        let res_original: usize = test.iter().map(|st| get_code(st)).sum();

        assert_eq!(res_encode - res_original, 19);
    }
}

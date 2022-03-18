fn main() {
    let input = "iwrupvqb";

    let mut ending = 1;
    loop {
        let i = format!("{}{}", input, ending);
        //let d = format!("{}{}", input, ending);
        let output = md5::compute(i);
        let out_str = format!("{:x}", output);
        //println!("Input {}, Output:{:x}", d, output);
        if out_str.starts_with("000000") {
            println!("{}", ending);
            break;
        }
        ending += 1;
    }
}

use std::io;

pub fn read_input_f64() -> f64{
    let mut input = String::new();
    read_input(&mut input);
    let val: f64 = parse_input_f64(&input);
    return val;
}

pub fn read_input_i64() -> i64{
    let mut input = String::new();
    read_input(&mut input);
    let val: i64 = parse_input_i64(&input);
    return val;
}

pub fn read_input_u64() -> u64{
    let mut input = String::new();
    read_input(&mut input);
    let val: u64 = parse_input_u64(&input);
    return val;
}

pub fn read_input_usize() -> usize{
    let mut input = String::new();
    read_input(&mut input);
    let val: usize = parse_input_usize(&input);
    return val;
}

pub fn read_input(string: &mut String){
    io::stdin().read_line(string).expect("Failed to read line");
}

pub fn parse_input_f64(input: &String) -> f64{
    return input.trim().parse().expect("Wanted a number");
}

pub fn parse_input_i64(input: &String) -> i64{
    return input.trim().parse().expect("Wanted a number");
}

pub fn parse_input_u64(input: &String) -> u64{
    return input.trim().parse().expect("Wanted a number");
}

pub fn parse_input_usize(input: &String) -> usize{
    return input.trim().parse().expect("Wanted a number");
}


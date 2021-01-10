use std::io;
fn main() {

    // Get input
    println!("Type a number of data: ");
    let mut input = String::new();
    read_input(&mut input);
    let n: usize = parse_input_usize(&input);
    let mut data = Vec::new();

    println!("Type each data: ");
    for _i in 0..n {
        let mut input = String::new();
        read_input(&mut input);
        let val: u64 = parse_input_u64(&input);
        data.push(val);
    }

    // Check input data
    println!("Input data:");
    for val in data.iter() {
        println!("{}", val);
    }


    println!("Type target data: ");
    let mut input = String::new();
    read_input(&mut input);
    let v: u64 = parse_input_u64(&input);

    // Initialization with false
    let mut exist = false;
    for val in data.iter() {
        if val == &v {
            exist = true;
        }
    }

    if exist {
        println!("Yes");
    }
    else{
        println!("No");
    }
}

fn read_input(string: &mut String){
    io::stdin().read_line(string).expect("Failed to read line");
}

fn parse_input_usize(input: &String) -> usize{
    return input.trim().parse().expect("Wanted a number");
}

fn parse_input_u64(input: &String) -> u64{
    return input.trim().parse().expect("Wanted a number");
}
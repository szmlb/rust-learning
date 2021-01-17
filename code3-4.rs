use std::io;
fn main() {

    // Get input
    println!("Type a number of data: ");
    let n: usize = read_input_usize();

    println!("Type a minimum sum number: ");
    let k: u64 = read_input_u64();

    println!("Type each data: ");
    let mut data_a = Vec::with_capacity(n);
    let mut data_b = Vec::with_capacity(n);
    data_a.resize(n, 0);
    data_b.resize(n, 0);
    for i in 0..n {data_a[i] = read_input_u64();}
    for i in 0..n {data_b[i] = read_input_u64();}

    // Check input data
    println!("Input data:");
    for val in data_a.iter() { println!("{}", val);}
    for val in data_b.iter() { println!("{}", val);}

    // Find minimum
    // Linear search
    let mut min_value = std::u64::MAX;
    for i in 0..n
    {
        for j in 0..n
        {
            // continue if sum is smaller than value k
            if data_a[i] + data_b[j] < k {
                continue;
            }

            // update minimum value
            if data_a[i] + data_b[j] < min_value {
                min_value = data_a[i] + data_b[j];
            }

        }
    }

    println!("{}", min_value);
}

fn read_input_u64() -> u64{
    let mut input = String::new();
    read_input(&mut input);
    let val: u64 = parse_input_u64(&input);
    return val;
}

fn read_input_usize() -> usize{
    let mut input = String::new();
    read_input(&mut input);
    let val: usize = parse_input_usize(&input);
    return val;
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
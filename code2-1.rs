use std::io;
fn main() {

    println!("Type loop number: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let n: u32 = input.trim().parse().expect("Wanted a number");
    println!("Input loop number: {}", n);

    let mut count = 0;
    print!("Single loop start...");
    for _i in 0..n {
        count = count + 1;
    }
    println!("end.");

}
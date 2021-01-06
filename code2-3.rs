use std::io;
fn main() {

    println!("Type a number: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let n: u32 = input.trim().parse().expect("Wanted a number");
    println!("Input number: {}", n);

    for _i in 0..=n/2 {
            println!("{}", 2 * _i);
    }

}
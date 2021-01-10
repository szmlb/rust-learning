use std::io;
fn main() {

    // Get input
    println!("Type a number of pair: ");
    let mut input = String::new();
    read_input(&mut input);
    let n: usize = parse_input_usize(&input);
    let mut data = Vec::new();

    for _i in 0..n {
        let mut input_x = String::new();
        let mut input_y = String::new();
        read_input(&mut input_x);
        read_input(&mut input_y);
        let x_in: f64 = parse_input_f64(&input_x);
        let y_in: f64 = parse_input_f64(&input_y);
        let pair = (x_in, y_in);
        
        data.push(pair);
    }

    // Check input data
    println!("Input data:");
    for pair in data.iter() {
        println!("({}, {})", pair.0, pair.1)
    }

    // Initialization with large value
    let mut minimum_dist = std::f64::MAX;

    // Using index access
    for i in 0..n {
        for j in i+1..n {
            // distance between (x[i],  y[i]) and (x[j], y[j])
            let dist_i_j: f64 = calc_dist(data[i].0,  data[i].1,  data[j].0,  data[j].1);

            // compare dist_i_j with a tentative minimum minimum_dist
            if dist_i_j < minimum_dist {
                minimum_dist = dist_i_j;
            }

        }
    }

    println!("Minimum distance: {}",  minimum_dist);

}

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64{
    return ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
}

fn read_input(string: &mut String){
    io::stdin().read_line(string).expect("Failed to read line");
}

fn parse_input_usize(input: &String) -> usize{
    return input.trim().parse().expect("Wanted a number");
}

fn parse_input_f64(input: &String) -> f64{
    return input.trim().parse().expect("Wanted a number");
}

use std::io;
fn main() {

    // Get input
    println!("Type a number of pair: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Wanted a number");

    let mut data = Vec::new();

    for _i in 0..n {
        let mut input_x = String::new();
        let mut input_y = String::new();
        io::stdin().read_line(&mut input_x).expect("Failed to read line");
        io::stdin().read_line(&mut input_y).expect("Failed to read line");

        let x_in: f64 = input_x.trim().parse().expect("Wanted a number");
        let y_in: f64 = input_y.trim().parse().expect("Wanted a number");
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
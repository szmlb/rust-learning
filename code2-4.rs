use std::io;
fn main() {

    // Get input
    println!("Type a number of pair: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Wanted a number");

    let mut x = Vec::new();
    let mut y = Vec::new();

    for _i in 0..n {
        let mut input_x = String::new();
        let mut input_y = String::new();
        io::stdin().read_line(&mut input_x).expect("Failed to read line");
        io::stdin().read_line(&mut input_y).expect("Failed to read line");

        let x_in: f64 = input_x.trim().parse().expect("Wanted a number");
        let y_in: f64 = input_y.trim().parse().expect("Wanted a number");
        
        x.push(x_in);
        y.push(y_in);

    }

    println!("Input data:");
    for i in 0..n {
        println!("({}, {})", x[i as usize], y[i as usize])
    }

    // Initialization with large value
    let mut minimum_dist = 100000000.0;

    for i in 0..n {
        for j in i+1..n {
            // distance between (x[i],  y[i]) and (x[j], y[j])
            let dist_i_j: f64 = calc_dist(x[i as usize],  y[i as usize],  x[j as usize],  y[j as usize]);

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

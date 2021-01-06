use std::io;
fn main() {

    // Get input
    println!("Type a number: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let n: u32 = input.trim().parse().expect("Wanted a number");

    let mut x = Vec::new();
    let mut y = Vec::new();

    for _i in 0..n {
        let mut input_x = String::new();
        let mut input_y = String::new();
        io::stdin().read_line(&mut input_x).expect("Error");
        io::stdin().read_line(&mut input_y).expect("Error");

        let _x: f64 = input.trim().parse().expect("Wanted a number");
        let _y: f64 = input.trim().parse().expect("Wanted a number");

        x.push(_x);
        y.push(_y);

    }

    // Initialization with large value
    let mut minimum_dist = 100000000.0;

    for _i in 0..n {
        for _j in _i+1..n {
            // distance between (x[i],  y[i]) and (x[j], y[j])
            let dist_i_j: f64 = calc_dist(x[_i as usize],  y[_i as usize],  x[_j as usize],  y[_j as usize]);
            println!("{}", dist_i_j);

            // compare dist_i_j with a tentative minimum minimum_dist
            if dist_i_j < minimum_dist {
                minimum_dist = dist_i_j;
            }

        }
    }

    println!("{}",  minimum_dist);

}

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64{

    return ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();

}

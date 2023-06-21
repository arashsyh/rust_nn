use std::f64::consts::E;

fn main() {
     // Defining a nested vector let xs = vec![vec![1, 2, 3]];
     println!("{}", nonlin(2.1, true));
}

fn nonlin(x: f64, derive: bool) -> f64 {
     if derive == false {
          return x * (1.0 - x)
     }
     else {
          return 1.0 / (1.0 + E.powf(-x))
     }
}
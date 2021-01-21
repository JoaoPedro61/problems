use std::io;


const PI: f64 = 3.14159;

fn main() {
  let mut inputA = String::new();
  
  io::stdin().read_line(&mut inputA);

  let r: f64 = inputA.trim().parse().unwrap();

  print!("A={:.4}\n", (PI * f64::powi(r, 2)));
}

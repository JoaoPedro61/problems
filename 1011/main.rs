use std::io;


const PI: f64 = 3.14159;


fn main() {
  let mut inputA = String::new();
  
  io::stdin().read_line(&mut inputA);

  let r: f64 = inputA.trim().parse().unwrap();

  print!("VOLUME = {:.3}\n", ((4.0 / 3.0) * PI * (f64::powi(r, 3))));
  
}

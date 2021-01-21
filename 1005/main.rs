use std::io;

const P1: f64 = 3.5;
const P2: f64 = 7.5;

fn main() {
  let mut inputA = String::new();
  let mut inputB = String::new();
  
  io::stdin().read_line(&mut inputA);
  io::stdin().read_line(&mut inputB);
  
  let a: f64 = inputA.trim().parse().unwrap();
  let b: f64 = inputB.trim().parse().unwrap();

  let m: f64 = ((a * P1) + (b * P2)) / (P1 + P2);
  
  print!("MEDIA = {:.5}\n", m);
}

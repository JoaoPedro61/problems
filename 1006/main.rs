use std::io;

const P1: f64 = 2.0;
const P2: f64 = 3.0;
const P3: f64 = 5.0;

fn main() {
  let mut inputA = String::new();
  let mut inputB = String::new();
  let mut inputC = String::new();
  
  io::stdin().read_line(&mut inputA);
  io::stdin().read_line(&mut inputB);
  io::stdin().read_line(&mut inputC);
  
  let a: f64 = inputA.trim().parse().unwrap();
  let b: f64 = inputB.trim().parse().unwrap();
  let c: f64 = inputC.trim().parse().unwrap();

  let m: f64 = ((a * P1) + (b * P2) + (c * P3)) / (P1 + P2 + P3);
  
  print!("MEDIA = {:.1}\n", m);
}

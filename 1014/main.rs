use std::io;



fn main() {
  let mut inputA = String::new();
  let mut inputB = String::new();
  
  io::stdin().read_line(&mut inputA);
  io::stdin().read_line(&mut inputB);

  let a: f64 = inputA.trim().parse().unwrap();
  let b: f64 = inputB.trim().parse().unwrap();

  let consume: f64 = a / b;

  print!("{:.3} km/l\n", consume);
}

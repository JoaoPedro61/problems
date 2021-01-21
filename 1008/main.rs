use std::io;


fn main() {
  let mut inputA = String::new();
  let mut inputB = String::new();
  let mut inputC = String::new();
  
  io::stdin().read_line(&mut inputA);
  io::stdin().read_line(&mut inputB);
  io::stdin().read_line(&mut inputC);
  
  let a: i32 = inputA.trim().parse().unwrap();
  let b: i32 = inputB.trim().parse().unwrap();
  let c: f64 = inputC.trim().parse().unwrap();

  let s: f64 = (b as f64 * c);
  // Or => let s: f64 = (f64::from(b) * c);
  
  print!("NUMBER = {}\n", a);
  print!("SALARY = U$ {:.2}\n", s);
}

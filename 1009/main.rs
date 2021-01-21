use std::io;


fn main() {
  let mut inputA = String::new();

  let mut inputB = String::new();
  let mut inputC = String::new();
  
  io::stdin().read_line(&mut inputA);
  io::stdin().read_line(&mut inputB);
  io::stdin().read_line(&mut inputC);

  let _a: String = inputA.trim().parse().unwrap();

  let b: f64 = inputB.trim().parse().unwrap();
  let c: f64 = inputC.trim().parse().unwrap();

  let s: f64 = (b + ((c / 100.0) * 15.0));
  
  print!("TOTAL = R$ {:.2}\n", s);
}

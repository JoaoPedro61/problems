use std::io;



fn split(string: String) -> Vec<f64> {
  let source: String = string.clone().trim().parse().unwrap();

  let splitter: Vec<f64> = source.split(' ')
    .map(|x| x.trim())
    .filter(|x| !x.is_empty())
    .map(|x| x.parse().unwrap())
    .collect();

  splitter
}


fn main() {
  let mut inputA = String::new();
  let mut inputB = String::new();
  
  io::stdin().read_line(&mut inputA);
  io::stdin().read_line(&mut inputB);

  let p1 = split(inputA);
  let p2 = split(inputB);
  
  let d: f64 = (f64::powi(p2[0] - p1[0], 2) + f64::powi(p2[1] - p1[1], 2)).sqrt();

  print!("{:.4}\n", d);
}

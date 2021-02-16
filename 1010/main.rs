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

  let a = split(inputA);
  let b = split(inputB);

  print!("VALOR A PAGAR: R$ {:.2}\n", ((a[1] * a[2]) + (b[1] * b[2])));
  
}

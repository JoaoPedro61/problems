use std::io;



const PI: f64 = 3.14159;


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
  
  io::stdin().read_line(&mut inputA);

  let values = split(inputA);

  let a: f64 = values[0];
  let b: f64 = values[1];
  let c: f64 = values[2];

  let tri = (a * c) / 2.0_f64;
  let cir = PI * f64::powi(c, 2);
  let tra = (c * (a + b)) / 2.0_f64;
  let qua = b * b;
  let ret = a * b;

  print!("TRIANGULO: {:.3}\n", tri);
  print!("CIRCULO: {:.3}\n", cir);
  print!("TRAPEZIO: {:.3}\n", tra);
  print!("QUADRADO: {:.3}\n", qua);
  print!("RETANGULO: {:.3}\n", ret);
}

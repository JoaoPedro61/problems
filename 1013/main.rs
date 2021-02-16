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

fn get_greatter(a: f64, b: f64) -> f64 {
  (a + b + (a - b).abs()) / 2.0
}

fn main() {
  let mut inputA = String::new();
  
  io::stdin().read_line(&mut inputA);

  let values = split(inputA);

  let a: f64 = values[0];
  let b: f64 = values[1];
  let c: f64 = values[2];

  let greatter = get_greatter(a, get_greatter(b, c));

  print!("{} eh o maior\n", greatter);
}

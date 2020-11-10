use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = parse_config(&args);

    let e = (1. + 1. / n as f64).powf(n as f64);

    println!("{}", e);
}

fn parse_config(args: &[String]) -> usize {
    let digits = &args[1];
    digits.parse::<usize>().unwrap()
}
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let iterations = parse_config(&args);

    let mut denominator: f64 = 1.;
    let mut pi = 0.;
    for _ in 1..iterations  {
        pi = pi + (4. / denominator);
        denominator = -1. * denominator.signum() * (denominator.abs() + 2.);
    }

    println!("{}", pi);
}

fn parse_config(args: &[String]) -> usize {
    let digits = &args[1];
    digits.parse::<usize>().unwrap()
}

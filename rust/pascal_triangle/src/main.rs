use std::io;

fn main() {
    let reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input)
          .ok()
          .expect("Failed to read k");

    let k = input.trim().parse().ok().expect("Please enter a number");
    pascal_triangle(k);
}


fn pascal_triangle(k: usize) {
    let mut matrix = vec![0; k * k];
    for i in 0..k {
        for j in 0..k {
            let idx = j + i * k;
            if j == 0 {
                matrix[idx] = 1;
            } else if j < i {
                matrix[idx] = matrix[j - 1 + (i - 1) * k] + matrix[j + (i - 1) * k];
            }

            if i == j {
                matrix[idx] = 1;
            }

            print!("{:?} ", matrix[idx]);

            if i == j {
              break;
            }
        }
        println!("");
    }    
}
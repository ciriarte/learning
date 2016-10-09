use std::io;

fn main() {
    let reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input)
          .ok()
          .expect("Failed to read n");

    let n = input.trim().parse().ok().expect("Please enter a number greater or equal to zero!");

    if n < 1 {
        println!("Order must be greater than zero");
    }

    if n > 5 {
        println!("{} is too big to display in a terminal", n);
    }

    let mut sierpinski_triangle = SierpinskiTriangle::with_iterations(n);

    sierpinski_triangle.display();
}

struct SierpinskiTriangle {
    iterations: usize,
    b: Vec<bool>
}

impl SierpinskiTriangle {
     pub fn with_iterations(n: u32) -> SierpinskiTriangle {
         let len: usize = 1 << (n+1);
         let mut s = SierpinskiTriangle {
             iterations: len,
             b: vec![false; len + 1] 
         };
         s.b[len >> 1] = true;
         s
     }

     pub fn display(&mut self) {
        for j in 0..(self.iterations / 2) {
            for i in 0..self.b.len() {
              print!("{0}", if self.b[i] { "*" } else { " " });
            }
            println!("");
            self.next_gen();
        }
     }

     fn next_gen(&mut self) {
         let mut next = vec![false; self.b.len()];
         for i in 0..self.b.len() {
             if self.b[i] {
                 next[i - 1] = next[i - 1] ^ true;
                 next[i + 1] = next[i + 1] ^ true;
             }
         }
         self.b = next;
     }     
}

extern crate rand;

use std::env;
use rand::{Rng, SeedableRng, StdRng};

const N: isize = 1;
const S: isize = 2;
const E: isize = 4;
const W: isize = 8;

fn main() {
    let width    = env::args().nth(1).unwrap_or("10".to_string()).parse().ok().unwrap_or(10);
    let height   = env::args().nth(2).unwrap_or(width.to_string()).parse().ok().expect("Please enter a valid height");
    //let seed     = env::args().nth(3).unwrap_or("10".to_string()).parse().ok().unwrap_or(1_f64);

    let zeed: &[_] = &[255, 255, 255, 255];
    let mut rng: StdRng = SeedableRng::from_seed(zeed);

    let mut maze = Maze::new(width, height, rng);
    maze.carve_passages_from(0, 0);
    maze.display();

    println!("width: {0}, height: {1}, seed: {2}", width, height, rng.gen::<f64>());    
}

  fn dx(direction: isize) -> isize {
      match direction {
          E => 1,
          W => -1,
          _ => 0
      }
  }

  fn dy(direction: isize) -> isize {
      match direction {
          N => -1,
          S => 1,
          _ => 0
      }
  }

  fn opposite(direction: isize) -> isize {
      match direction {
          N => S,
          S => N,
          W => E,
          E => W,
          _ => 0
      }      
  } 

struct Maze {
    grid: Vec<Vec<isize>>,
    width: usize,
    height: usize,
    rng: StdRng,
}

impl Maze {

  pub fn new(width: usize, height: usize, rng: StdRng) -> Maze {
      Maze {
          grid: vec![vec![0; width]; height],
          width: width,
          height: height,
          rng: rng
      }
  }

  pub fn display(&self) {
      println!(" {0}", (0..self.width * 2 - 1).map(|_| "_").collect::<String>());
      for y in 0..self.height {
          print!("|");
          for x in 0..self.width {
              print!("{0}", if self.grid[y][x] & S != 0 { " " } else { "_" });
              if self.grid[y][x] & E != 0 {
                  print!("{0}", if (self.grid[y][x] | self.grid[y][x + 1]) & S != 0 { " " } else { "_" } );
              } else {
                  print!("|");
              }
          }
          println!("");
      }
  }

  fn carve_passages_from(&mut self, cx: usize, cy: usize) {
      let mut directions = vec![N, S, E, W];
      directions.sort_by(|a, _| self.rng.gen::<isize>().cmp(&a));

      for direction in directions {
          let nx = cx as isize + dx(direction);
          let ny = cy as isize + dy(direction);


          if ny >= 0 && nx >= 0 {
            let nx = nx as usize;
            let ny = ny as usize;

            if ny < self.grid.len() && 
               nx < self.grid[ny].len() &&
               self.grid[ny][nx] == 0 {
                self.grid[cy][cx] |= direction;
                self.grid[ny][nx] |= opposite(direction);
                self.carve_passages_from(nx, ny);
            }
          }
      }
  }
}

use std::io;

fn main() {
    let reader = io::stdin();
    let mut count_and_offset = String::new();
    reader.read_line(&mut count_and_offset)
        .ok()
        .expect("Failed to read count and offset");

    let count_and_offset: Vec<usize> = count_and_offset.split_whitespace()
        .map(|s| s.parse().ok().expect("Please enter a positive number"))
        .collect::<Vec<_>>();

    let mut input = String::new();
    reader.read_line(&mut input)
        .ok()
        .expect("Failed to read array of numbers to rotate");

    let input: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().ok().expect("Please enter a number!"))
        .collect::<Vec<_>>();

    let res = rotate_left(input, count_and_offset[1]);

    println!("{:?}", res);
}

fn rotate_left(vec: Vec<i32>, mut offset: usize) -> Vec<i32> {
    println!("input: {0:?}, offset: {1}", vec, offset);

    let length = vec.len();

    offset = match offset.checked_rem(length) {
        Some(k) => k,
        None => panic!("Input length must be greater than zero"),
    };

    if offset == 0 {
        return vec;
    }

    let mut res = vec![0; 5];
    for i in 0..length {
        let new_pos = match i.checked_sub(offset) { 
            Some(k) => k,
            None => i + length.wrapping_sub(offset),
        };
        res[new_pos] = vec[i];
        println!("{0:?} => i: {1}, new_pos: {2:?}", res, i, new_pos);
    }

    res
}

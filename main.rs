use std::io;

fn main() {

    println!("Enter file path");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    let v: Vec<u8> = std::fs::read(input).expect("Failed to read path");

    let mut count = [0u64; 256];

    for &byte in &v {

        count[byte as usize] += 1

    }

    println!("The total amount of bytes are {}", v.len());

    let total = v.len() as f64;
    let mut entropy = 0.0;

    for &c in &count {

        if c == 0 {

            continue;

        }

        let p = c as f64 / total;
        entropy -= p * p.log2()


    }

    println!("The entropy is {}", entropy);

}
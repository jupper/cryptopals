extern crate num_traits;

use std::io;
use num_traits::pow;

fn main() {
    let mut first_string = String::new();
    let mut last_string = String::new();
    let mut first_string_bin = String::new();
    let mut last_string_bin = String::new();
    let mut result = String::new();
    println!("Please enter the first string: ");
    io::stdin().read_line(&mut first_string).expect("Failed to read line");
    println!("Please enter the last string: ");
    io::stdin().read_line(&mut last_string).expect("Failed to read line");
    first_string_bin = string_to_hex(first_string.trim());
    last_string_bin = string_to_hex(last_string.trim());
    result = xor(first_string_bin, last_string_bin);
    println!("Result: {}", bin_to_hex(result));
}

fn string_to_hex(text: &str) -> String {
	let mut hex_string = String::new();
	for c in text.chars() {
		hex_string.push_str(&hex_to_bits(c));
	}
	hex_string
}

fn hex_to_bits(c: char) -> String {
	let mut result = String::new();
	let x = c.to_digit(16).unwrap();
	let mut tmp = x;
	while tmp != 0 {
		result = (tmp % 2).to_string() + &result;
		tmp = tmp/2;
	}
	while result.len() < 4 {
		result = "0".to_string() + &result;
	}
	result
}

fn xor(first: String, last: String) -> String {
	let mut result = String::new();
	for i in 0..first.len() {
		if &first[i..(i+1)] == "1" && &last[i..(i+1)] == "1" {
			result = result + &"0".to_string(); 
		} else if &first[i..(i+1)] == "1" || &last[i..(i+1)] == "1" {
			result = result + &"1".to_string();			
		} else {
			result = result + &"0".to_string();
		}
	}	
	result
}

fn bin_to_hex(bin: String) -> String {
	let mut result = String::new();
	let chars: Vec<char> = "0123456789abcdef".chars().collect();
	for i in 0..(bin.len()/4) {
		let mut res = 0;
		let mut counter = 0;
		for c in bin[(i*4)..((i*4)+4)].chars().rev() {
			let x = c.to_digit(10).unwrap();			
			if x == 1 {
				res += pow(2i8, counter);
			}
			counter += 1;			
		}
		result.push(chars[res as usize]);
	}
	result
}
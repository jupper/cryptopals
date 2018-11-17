extern crate num_traits;

use std::io;
use num_traits::pow;

const FREQUENCY: &str = "zqxjkvbpygfwmucldrhsnioate";

fn main() {
    let mut hex_string = String::new();
    let mut bin_string = String::new();
    let mut vec_bin: Vec<String> = Vec::new();
    let mut vec_plain: Vec<String> = Vec::new();
    let mut vec_count: Vec<u32> = Vec::new();
    let mut position = 0;
    let mut counter = 0;
    let mut value: u32 = 0;
    println!("Please enter the string: ");
    io::stdin().read_line(&mut hex_string).expect("Failed to read line");
    bin_string = string_to_hex(hex_string.trim());
    for i in 0..256 {
    	let mut tmp = bin_string.clone();
    	vec_bin.push(xor(tmp, i));
    }
	for s in vec_bin {
		let mut vec_utf8: Vec<u8> = Vec::new();
		for i in 0..(s.len()/8) {
			vec_utf8.push(bits_to_int(s[(i*8)..((i*8)+8)].to_string()));
		}
		let mut tmp = String::new();
		tmp.push_str(&String::from_utf8_lossy(&vec_utf8));
		vec_plain.push(tmp);
	}
	let vec = vec_plain.clone();
	for s in vec_plain {
		vec_count.push(count_frequency(s));
	}
	for u in vec_count {
		if u > value {
			value = u;
			position = counter;
		}
		counter += 1;
	}
	println!("{}",vec[position]); 
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

fn xor(s: String, u: u16) -> String {
	let mut result = String::new();
	let bin = dec_to_bin(u);
	for i in 0..(s.len()/8) {
		let mut tmp = bin.clone();
		result = result + &single_byte_xor(s[(i*8)..((i*8)+8)].to_string(), tmp);
	}
	result
}

fn dec_to_bin(u: u16) -> String {
	let mut result = String::new();	
	let mut tmp = u;
	while tmp != 0 {
		result = (tmp % 2).to_string() + &result;
		tmp = tmp/2;
	}
	while result.len() < 8 {
		result = "0".to_string() + &result;
	}
	result
}

fn single_byte_xor(s: String, u: String) -> String {
	let mut result = String::new();
	for i in 0..s.len() {
		if &s[i..(i+1)] == "1" && &u[i..(i+1)] == "1" {
			result = result + &"0".to_string(); 
		} else if &s[i..(i+1)] == "1" || &u[i..(i+1)] == "1" {
			result = result + &"1".to_string();			
		} else {
			result = result + &"0".to_string();
		}
	}	
	result
}

fn bits_to_int(s: String) -> u8 {
	let mut result: u8 = 0;
	let mut counter = 0; 
	for c in s.chars().rev() {
		let x = c.to_digit(10).unwrap();
		if x == 1 {
			result += pow(2u8, counter);
		}
		counter += 1;
	}
	result
}

fn count_frequency(s: String) -> u32 {
	let mut result: u32 = 0;
	for c in s.chars() {
		let mut counter = 0;
		for f in FREQUENCY.chars() {
			if c == f {
				result += counter;
				break;
			}
			counter += 1;
		}
	}
	result
}

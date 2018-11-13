extern crate num_traits;

use std::io;
use num_traits::pow;

const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
	let mut text_to_convert = String::new();
    println!("Please enter the string to convert!");
    io::stdin().read_line(&mut text_to_convert).expect("Failed to read line");
    println!("Base64: {}", bits_to_base64(string_to_base64(text_to_convert.trim())));
}

fn string_to_base64(text: &str) -> String {
	let mut base64_string = String::new();
	for c in text.chars() {
		base64_string.push_str(&hex_to_bits(c));
	}
	base64_string
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

fn bits_to_base64(s: String) -> String {
	let mut result = String::new();
	let mut new_s = s;
	let chars: Vec<char> = BASE64_ALPHABET.chars().collect();
	let mut missing_bytes = (new_s.len() / 8) % 3;
	if missing_bytes == 2 {
		missing_bytes = 1;
	} else if missing_bytes == 1 {
		missing_bytes = 2;
	}
	for _i in 0..missing_bytes {
		new_s = new_s + &"00".to_string();
	}
	for i in 0..(new_s.len()/6){
		let x = bits_to_int(new_s[(i*6)..((i*6)+6)].to_string()) as usize;
		result.push(chars[x]);
	}
	for _i in 0..missing_bytes {
		result.push_str("=");
	}
	result
}

fn bits_to_int(s: String) -> i8 {
	let mut result = 0;
	let mut counter = 0; 
	for c in s.chars().rev() {
		let x = c.to_digit(10).unwrap();
		if x == 1 {
			result += pow(2i8, counter);
		}
		counter += 1;
	}
	result
}
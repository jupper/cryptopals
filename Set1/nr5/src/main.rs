extern crate hex_slice;

use std::io;
use hex_slice::AsHex;

fn main() {
	let mut string = String::new();
	let mut key = String::new();
	let mut counter = 0;
	let mut dec_string: Vec<u8>  = Vec::new();
	let mut dec_key: Vec<u8> = Vec::new(); 
    println!("Please enter the string: ");
    io::stdin().read_line(&mut string).expect("Failed to read line");
    println!("Please enter the key: ");
    io::stdin().read_line(&mut key).expect("Failed to read line");
    dec_string = string.trim().to_string().into_bytes();
    dec_key = key.trim().to_string().into_bytes();
    while dec_string.len() % dec_key.len() != 0 {
    	counter += 1;
    	dec_string.push(0);
    }
    for i in 0..(dec_string.len()/dec_key.len()) {
    	for j in 0..dec_key.len() {
    		dec_string[i*dec_key.len() + j] ^= dec_key[j];
    	}
    }
    for _i in 0..counter {
    	dec_string.pop();
    }
    println!("{:x}", dec_string.as_hex());
}

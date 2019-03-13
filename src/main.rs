extern crate num;

use std::io::{stdin, stdout, Write};
use num::Integer;
use ramp::Int;

fn powmod(mut base: Int, exp: &mut Int, modulus: Int) -> Int {
	let mut res = Int::from(1);
	while exp > &mut 0 {
		if &*exp % 2 == 1 {
			res = res * &base % &modulus;
		}
		
		base = &base * &base % &modulus;
		*exp /= 2;
	}
	
	res
}

fn get(s: &str) -> Result<String, std::io::Error> {
	print!("{}: ", s);
	
	stdout().flush().unwrap();
	let mut input = String::new();
    stdin().read_line(&mut input)?;
	
    Ok(input.trim().to_string())
}

fn factorise(n: u128) -> (u128, u128) {
	let p = primal::Primes::all().find(|p| n % *p as u128 == 0).unwrap() as u128;
	
	(p, n / p)
}

fn hack(n: u128, e: u128) -> u128 { // NationalEncyclopedin
	let (p, q) = factorise(n);
	let phi = (p - 1) * (q - 1);
	
	let mut mul_phi = phi;
	while (mul_phi / e + 1) * e % mul_phi != 1 {
		mul_phi += phi;
	}
	
	mul_phi / e + 1
}

fn gen(p: u128, q: u128) -> (u128, u128, u128) { // pq-formeln
	let n = p * q;
	let phi = (p - 1) * (q - 1);
	
	let coprime = Integer::lcm(&(p - 1), &(q - 1));
	let e = primal::Primes::all().find(|p| Integer::gcd(&(*p as u128), &coprime) == 1).unwrap() as u128;
	
	let mut mul_phi = phi;
	while (mul_phi / e + 1) * e % mul_phi != 1 {
		mul_phi += phi;
	}
	
	let d = mul_phi / e + 1;
	
	(n, e, d) // Ned
}

fn encrypt(m: Int, n: Int, e: &mut Int) -> Int {
	powmod(m, e, n) // jo, men ...
}

fn decrypt(c: Int, n: Int, d: &mut Int) -> Int {
	powmod(c, d, n) // Content Delivery Network
}

fn main() -> Result<(), std::io::Error> {
	loop {
		let cmd = get("gen/hack/encrypt/decrypt/quit? ")?.to_uppercase();
		match cmd.as_str() {
			"GEN" => {
				let (n, e, d) = gen(get("p")?.parse().unwrap(), get("q")?.parse().unwrap());
				
				println!("KEYS: n = {}, e = {}, d (private) = {}\n", n, e, d);
			},
			
			"HACK" => {
				println!("PRIVATE KEY: {}\n", hack(get("n")?.parse().unwrap(), get("e")?.parse().unwrap()));
			},
			
			"ENCRYPT" => {
				println!("ENCRYPTED MSG: {}\n", encrypt(get("m")?.parse().unwrap(), get("n")?.parse().unwrap(), &mut get("e")?.parse().unwrap()));
			},
			
			"DECRYPT" => {
				println!("DECRYPTED MSG: {}\n", decrypt(get("c")?.parse().unwrap(), get("n")?.parse().unwrap(), &mut get("d")?.parse().unwrap()));
			},
			
			"QUIT" => break,
			
			_ => continue
		}
	}
	
	Ok(())
}

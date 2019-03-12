extern crate num;

use std::io::{stdin, stdout, Write};
use num::Integer;

fn powmod(mut base: usize, mut exp: usize, modulus: usize) -> usize {
	let mut res = 1;
	while exp > 0 {
		if exp % 2 == 1 {
			res = res * base % modulus;
		}
		
		base = base * base % modulus;
		exp /= 2;
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

fn factorise(n: usize) -> (usize, usize) {
	let p = primal::Primes::all().find(|p| n % *p == 0).unwrap();
	
	(p, n / p)
}

fn hack(n: usize, e: usize) -> usize { // NationalEncyclopedin
	let (p, q) = factorise(n);
	let phi = (p - 1) * (q - 1);
	
	let mut d = 2;
	while d * e % phi != 1 {
		d += 1;
	}
	
	d
}

fn gen(p: usize, q: usize) -> (usize, usize, usize) { // pq-formeln
	let n = p * q;
	let phi = (p - 1) * (q - 1);
	
	let coprime = Integer::lcm(&(p - 1), &(q - 1));
	let e = primal::Primes::all().find(|p| Integer::gcd(p, &coprime) == 1).unwrap();
	
	let mut d = 2;
	while d * e % phi != 1 {
		d += 1;
	}
	
	(n, e, d)
}

fn encrypt(m: usize, n: usize, e: usize) -> usize {
	powmod(m, e, n) // jo, men ...
}

fn decrypt(c: usize, n: usize, d: usize) -> usize {
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
				println!("ENCRYPTED MSG: {}\n", encrypt(get("m")?.parse().unwrap(), get("n")?.parse().unwrap(), get("e")?.parse().unwrap()));
			},
			
			"DECRYPT" => {
				println!("DECRYPTED MSG: {}\n", decrypt(get("c")?.parse().unwrap(), get("n")?.parse().unwrap(), get("d")?.parse().unwrap()));
			},
			
			"QUIT" => break,
			
			_ => continue
		}
	}
	
	Ok(())
}

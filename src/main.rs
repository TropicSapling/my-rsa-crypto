use std::io::{stdin, stdout, Write};

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

fn hack(e: usize, n: usize) -> usize {
	let (p, q) = factorise(n);
	
	let org_phi = (p - 1) * (q - 1);
    let mut phi = org_phi;
/*    while (phi + 1) % e != 1 {
        phi += org_phi; // 123*d mod org_phi = 1
	}
    
    let d = (phi + 1) / e; */
	
	let mut d = 1;
	while e * d % phi != 1 {
		d += 1;
	}
	
	d
}

fn gen(p: usize, q: usize) -> (usize, usize, usize) {
	let n = p * q;
	
	// WIP
	
//	(n, e, d)
	(1, 2, 3)
}

fn encrypt(m: usize, n: usize, e: usize) -> usize {
	m.pow(e as u32) % n
}

fn decrypt(c: usize, n: usize, d: usize) -> usize {
	c.pow(d as u32) % n
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
				println!("PRIVATE KEY: {}\n", hack(get("e")?.parse().unwrap(), get("n")?.parse().unwrap()));
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

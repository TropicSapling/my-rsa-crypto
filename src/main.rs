use std::io::{stdin, stdout, Write};

fn get(s: &str) -> Result<usize, std::io::Error> {
	print!("{}: ", s);
	
	stdout().flush().unwrap();
	let mut input = String::new();
    stdin().read_line(&mut input)?;
	
    Ok(input.trim().parse().unwrap())
}

fn factorise(n: usize) -> (usize, usize) {
	let p = primal::Primes::all().find(|p| n % *p == 0).unwrap();
	
	(p, n / p)
}

fn main() -> Result<(), std::io::Error> {
	let (e, n, b) = (get("e")?, get("n")?, get("b")?);
	
    println!("{:?}", factorise(n));
	
	Ok(())
}

mod fib;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	if args.len() <= 1 {
		print!("usage: ./fib numbers");
	} else {
		for i in 1..args.len() {
			let val = &args[i];
			let num: i64 = val.parse::<i64>().unwrap();
			let res = fib::fibonacci(num);
			match res {
				Ok(val) => println!("fib {} = {}", num, val),
				Err(error) => println!("Error: {}", error),
			}
		}
	}
}

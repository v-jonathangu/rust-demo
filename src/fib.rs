pub fn fibonacci(n: i64) -> Result<i64, String> {

	if n < 0 {
		return Err(format!("{} is a negative number", n));
	} else if n <= 1 {
		return Ok(1);
	}

	let mut sum = 0;
	let mut last = 0;
	let mut curr = 1;
	for _i in 1..n {
		sum = last + curr;
		last = curr;
		curr = sum;
	}
	return Ok(sum);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fibonacci() -> Result<(), String> {
		let n = 20;
		let res = 6765;
		assert_eq!(fibonacci(n)?, res);
		Ok(())
	}
	#[test]
	fn test_negative_fail_fibonnacci() -> Result<(), String>{
		return match fibonacci(-1){
			Ok(_) => Err("negative should not be supported".to_string()),
			Err(_) => Ok(())
		};
	}
}
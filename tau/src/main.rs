use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;
use term_size;

fn compute_tau_digits(digits: u32) -> BigInt {
	// https://www.craig-wood.com/nick/articles/pi-chudnovsky/
	let error_digits = if digits > 10000 { digits.ilog10() + 3 } else { digits.ilog2() + 3 };
	let unit = 10.to_bigint().unwrap().pow(digits + error_digits);
	let mut a_sum = unit.clone();
	let mut a = unit.clone();
	let mut b_sum = 0.to_bigint().unwrap();
	let mut k = 1.to_bigint().unwrap();
	while !a.is_zero() {
		let denom = k.clone().pow(3) * 640320u64.pow(3);
		let numer = -24 * (6 * k.clone() - 5) * (2 * k.clone() - 1) * (6 * k.clone() - 1) * a;
		a = numer / denom;
		b_sum += a.clone() * k.clone();
		a_sum += a.clone();
		k += 1;
	}
	let tau_numerator = 426880 * (10005u64 * unit.pow(2)).sqrt() * unit;
	let tau_denominator = 13591409 * a_sum + 545140134 * b_sum;
	return tau_numerator / tau_denominator * 2 / 10u64.pow(error_digits);
}

fn main() {
	let (w, h) = term_size::dimensions().unwrap_or((80, 40));
	let tau = compute_tau_digits((w * h) as u32 * 79 / 100).to_string();
    let tau = " tau = 6.".to_owned() + tau.split_at(1).1;
	let r = (w.min((h - 1) * 2) / 2) as i32;
	let mut i: usize = 0;
	for y in 0..(h as i32) {
		for x in 0..(w as i32) {
			let current_r = (x - (w as i32) / 2).pow(2) + (2 * y - (h as i32)).pow(2);
			if current_r <= (r + 1) * r && ((r + 1) * r - current_r).rem_euclid(1000) < 500 {
				print!("{}", tau.get(i..(i + 1)).unwrap());
				i += 1;
			} else {
				print!(" ");
			}
		}
		println!();
	}
	// println!("tau = {}", compute_tau_digits(1000000) % 1000000);
}

#[cfg(test)]
mod tests {
	use crate::compute_tau_digits;
	#[test]
	fn up_to_1000_digits() {
		let correct_tau = include_str!("tau.txt").replace(".", "").replace("\n", "");
		for digits in 1..1000 {
			let tau = compute_tau_digits(digits).to_str_radix(10);
			assert_eq!(tau, correct_tau.split_at((1 + digits) as usize).0);
		}
	}
	#[test]
	fn about_10000_digits() {
		let correct_tau = include_str!("tau.txt").replace(".", "").replace("\n", "");
		for digits in [10000, 10009, 10018, 10257, 13994, 17686] {
			let tau = format!("{:06}", (compute_tau_digits(digits) % 1000000i32));
			let digits = digits as usize;
			let correct_tau = correct_tau.split_at(1 + digits).0.split_at(digits - 5).1;
			assert_eq!(tau, correct_tau);
		}
	}
	#[test]
	fn about_100000_digits() {
		let correct_tau = include_str!("tau.txt").replace(".", "").replace("\n", "");
		for digits in [100000, 112785] {
			let tau = format!("{:06}", (compute_tau_digits(digits) % 1000000i32));
			let digits = digits as usize;
			let correct_tau = correct_tau.split_at(1 + digits).0.split_at(digits - 5).1;
			assert_eq!(tau, correct_tau);
		}
	}
}

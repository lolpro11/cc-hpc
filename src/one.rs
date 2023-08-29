use num_bigint::{BigUint, ToBigUint};
use num_traits::Euclid;
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let mut buffer = String::new();
    if args.len() > 1 {
        buffer.push_str(&args[1].as_str())
    }


    let number: BigUint = buffer.trim().parse().expect("Failed to parse number");
    println!("Number read from commandline: {}", number);
    let num = number.clone();
    let mut s = "".to_string();
    let mut temp_num = num.clone();
    s.push_str(format!("{}", temp_num.to_str_radix(10)).as_str());
    while temp_num != 1.to_biguint().unwrap() {
        if temp_num.rem_euclid(&2.to_biguint().unwrap()) == 1.to_biguint().unwrap() {
            temp_num = ((3.to_biguint().unwrap() * temp_num) + 1.to_biguint().unwrap()) / 2.to_biguint().unwrap();
            s.push_str(format!(" {}", temp_num.to_str_radix(10)).as_str());
        } else {
            temp_num = temp_num / 2.to_biguint().unwrap();
            s.push_str(format!(" {}", temp_num.to_str_radix(10)).as_str());
        }
    }
    s.push_str("\n");
    println!("{}", s);
    Ok(())
}

use std::{fs::File, io::{Read, self}};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Euclid;

fn main() -> io::Result<()> {
    let mut file = File::open("number.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let number: BigUint = buffer.trim().parse().expect("Failed to parse number");

    println!("Number read from file: {}", number);
    let mut num: BigUint = number;
    let mut temp_num = num.clone();
    loop{
        temp_num = num.clone();
        print!("\n{}", temp_num);
        while temp_num != 1.to_biguint().unwrap() {
            if temp_num.rem_euclid(&2.to_biguint().unwrap()) == 1.to_biguint().unwrap() {
                temp_num = ((3.to_biguint().unwrap() * temp_num) + 1.to_biguint().unwrap()) / 2.to_biguint().unwrap();
                print!(" {}", temp_num);
            } else {
                temp_num = temp_num / 2.to_biguint().unwrap();
                print!(" {}", temp_num);
            }
            if temp_num <= num {
                break;
            }
        }
        num = num + 1.to_biguint().unwrap();
    }
    Ok(())
}

use std::{fs::{File, OpenOptions}, io::{Read, self, Write}};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Euclid;
use std::thread;
use std::fs;

fn main() -> io::Result<()> {
    let mut file = File::open("number.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    fs::create_dir_all("./numbers/")?;

    let number: BigUint = buffer.trim().parse().expect("Failed to parse number");
    println!("Number read from file: {}", number);
    let num_threads = 32;
    //let big: BigUint = BigUint::new([6, 5, 5, 3, 6].to_vec());
    //340282366920938463463374607431768211456
    let mut children = vec![];
    for i in 0..num_threads {
        let factor: BigUint = 134217728.to_biguint().unwrap();
        let sec: BigUint;
        if i == 0 {
            sec = 1.to_biguint().unwrap();
        } else {
            sec = (i.to_biguint().unwrap() * factor.clone()) + 1.to_biguint().unwrap();
        }
        let end: BigUint = (i + 1).to_biguint().unwrap() * factor.clone();
        children.push(thread::spawn(move || {
            let mut num = sec.clone();
            let file_path = format!("./numbers/{}-{}.txt", sec.to_str_radix(10).as_str(), end.to_str_radix(10).as_str());
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();
            let mut s = "".to_string();
            loop {
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
                    if temp_num <= num {
                        break;
                    }
                }
                if num >= end {
                    break;
                }
                s.push_str("\n");
                if num.clone() % 10000.to_biguint().unwrap() == 0.to_biguint().unwrap() {
                    file.write(s.as_bytes()).unwrap();
                    s = "".to_string()
                }
                num = num + 1.to_biguint().unwrap();
            }
        }));
    }
    for child in children {
        let _ = child.join();
    }
    Ok(())
}

use std::{fs::{File, OpenOptions}, io::{Read, self, Write}};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Euclid;
use std::thread;
use std::fs;

fn main() -> io::Result<()> {
    let mut file = File::open("number.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    fs::create_dir_all("./mersenne-numbers/")?;

    let number: BigUint = buffer.trim().parse().expect("Failed to parse number");

    println!("Number read from file: {}", number);
    let num_threads = 16 - 1;
    //let big: BigUint = BigUint::new([6, 5, 5, 3, 6].to_vec());
    //340282366920938463463374607431768211456
    let mut children = vec![];
    for i in 0..num_threads {
        let sec: BigUint;
        if i == 0 {
            sec = 2.to_biguint().unwrap();
        } else {
            sec = (i.to_biguint().unwrap() * 1_000.to_biguint().unwrap()) + 1.to_biguint().unwrap();
        }
        let end: BigUint = (i + 1).to_biguint().unwrap() * 1_000.to_biguint().unwrap();
        children.push(thread::spawn(move || {
            let mut iterator = sec.clone();
            let file_path = format!("./mersenne-numbers/{}-{}.txt", sec.to_str_radix(10).as_str(), end.to_str_radix(10).as_str());
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();
            let mut s = "".to_string();
            loop {
                let num =  2.to_biguint().unwrap().pow(iterator.clone().to_str_radix(10).to_string().parse::<u32>().unwrap()) - 1.to_biguint().unwrap();
                let mut temp_num = num.clone();
                let mut x = 0;
                while temp_num.rem_euclid(&2.to_biguint().unwrap()) != 0.to_biguint().unwrap() && temp_num != 1.to_biguint().unwrap() {
                    if temp_num.rem_euclid(&2.to_biguint().unwrap()) == 1.to_biguint().unwrap() {
                        temp_num = ((3.to_biguint().unwrap() * temp_num) + 1.to_biguint().unwrap()) / 2.to_biguint().unwrap();
                        x += 1;
                    } else {
                        break;
                    }
                    //if temp_num <= num {
                        //break;
                    //}
                }
                if iterator ==  end.clone() + 2.to_biguint().unwrap() {
                    break;
                }
                s.push_str(format!("{} {}\n", num.to_str_radix(10), x.to_string()).as_str());
                if iterator.clone() % 50.to_biguint().unwrap() == 0.to_biguint().unwrap() {
                    file.write(s.as_bytes()).unwrap();
                    s = "".to_string()
                }
                iterator += 1.to_biguint().unwrap();
            }
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    Ok(())
}

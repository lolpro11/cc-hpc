fn main() {
    for i in (u128::MIN + 1)..u128::MAX {
        let mut num = i;
        while num != 1 {
            if (num % 2) == 1 {
                num = ((3 * num) + 1)/2
            } else {
                num = num / 2
            }
        }
        println!("{} leads to 1 via CC" , i);
    }

}

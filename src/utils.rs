use rand::prelude::*;
use std::char;

pub fn random_layout() -> String {
    let mut temp: [i32; 6] = [1, 2, 3, 4, 5, 6];

    let mut ans = String::new();

    let mut rng = thread_rng();
    for i in 0..6 {
        let idx: i32 = rng.gen_range(0, 5 - i + 1);
        ans.push(char::from_digit(temp[idx as usize] as u32, 10).unwrap());

        temp.swap(idx as usize, 5 - i as usize);
    }

    ans
}
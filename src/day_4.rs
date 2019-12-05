use std::collections::HashMap;

pub fn solve(is_first_part: bool) {
    let mut password_num = 0;
    'pass_loop: for pass in LOW..HIGH {
        let pass_arr = num_to_array(pass);
        let mut two_same = false;
        let mut hashmap = HashMap::new();
        for i in 0..5 {
            if pass_arr[i] == pass_arr[i + 1] {
                two_same = true
            }
            if pass_arr[i] > pass_arr[i + 1] {
                continue 'pass_loop
            }
        }
        // count numbers
        for i in 0..=5 {
            match hashmap.get_mut(&pass_arr[i]) {
                Some(val) => {
                    *val += 1;
                },
                None => {
                    hashmap.insert(pass_arr[i], 1);
                }
            }
        }
        if !two_same {
            continue
        }
        let mut found_double = false;
        for (_key, val) in hashmap {
            if val == 2 {
                found_double = true;
            }
        }
        if !found_double && is_first_part {
            continue
        }

        password_num += 1;
    }
    println!("password_num = {}", password_num);
}

fn num_to_array(num: u32) -> Vec<u32> {
    let mut num = num;
    let mut vec = Vec::with_capacity(6);
    while num > 0 {
        vec.push(num % 10);
        num /= 10;
    }
    vec.reverse();
    vec
}

const LOW: u32 = 273025;
const HIGH: u32 = 767253;

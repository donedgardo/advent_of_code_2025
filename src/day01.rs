pub fn get_password(input: Vec<String>, start: u32) -> u32 {
    let mut password =  0;
    let mut pos = start as i32;
    for n in input {
        let direction = get_direction(&n);
        let qty = get_qty(&n);
        pos += direction as i32 * qty as i32;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            password += 1;
        }
    }
    password
}
pub fn get_password2(input: Vec<String>, start: u32) -> u32 {
    let mut password =  0;
    let mut pos = start as i32;
    for n in input {
        let direction = get_direction(&n);
        let qty = get_qty(&n);
        let range;
        if direction == -1 {
            range = pos + (direction as i32 * qty as i32)..pos;
        } else {
            range = (pos + 1)..(pos + (direction as i32 * qty as i32) + 1);
        }
        for tick in range {
            if tick%100 == 0 {
                password += 1;
            }
        }

        pos += direction as i32 * qty as i32;

    }
    password
}

fn get_qty(n: &String) -> u32 {
    n[1..].parse().unwrap()
}

fn get_direction(n: &String) -> i8 {
    let mut direction = 1;
    match n.chars().next() {
        None => {}
        Some(rotation) => {
            match rotation {
                'L' => direction = -1,
                _ => {}
            }
        }
    }
    direction
}

#[cfg(test)]
mod day01_test {
    use crate::day01::{get_password, get_password2};
    use crate::get_sample_input;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day01.txt");
        let answer = get_password(input, 50);
        assert_eq!(answer, 3);
    }

    #[test]
    fn sample_test_2() {
        let input = get_sample_input("day01.txt");
        let answer = get_password2(input, 50);
        assert_eq!(answer, 6);
    }
}

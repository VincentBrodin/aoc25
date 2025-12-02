fn main() {
    let input = include_str!("../input.txt");
    let rows = input.split('\n');
    let mut dial = 50;
    let mut zeros = 0;
    for row in rows {
        if row.is_empty() {
            break;
        }
        let (dir_str, times_str) = row.split_at(1);
        let times: i32 = times_str.parse().unwrap();
        // if dir_str == "L" {
        //     (dial, zeros) = rotate_left(dial, times, zeros);
        // } else {
        //     (dial, zeros) = rotate_right(dial, times, zeros);
        // }
        // // if dial == 0 {
        // //     zeros += 1;
        // // }
        // Super ugly code but it works!!!
        for _ in 0..times {
            if dir_str == "L" {
                dial -= 1;
            } else {
                dial += 1;
            }

            if dial == 0 {
                zeros += 1;
            } else if dial == -1 {
                dial = 99;
            } else if dial == 100 {
                zeros += 1;
                dial = 0;
            }
        }
    }
    println!("{zeros}");
}

fn rotate_left(dial: i32, times: i32, wraps: i32) -> (i32, i32) {
    if dial - times < 0 {
        rotate_left(99, (dial - times + 1).abs(), wraps + 1)
    } else {
        (dial - times, wraps)
    }
}

#[test]
fn rotate_left_test() {
    assert_eq!(rotate_left(50, 68, 0), (82, 1));
    assert_eq!(rotate_left(82, 30, 0), (52, 0));
}

fn rotate_right(dial: i32, times: i32, wraps: i32) -> (i32, i32) {
    if dial + times > 99 {
        rotate_right(0, (dial + times - 100).abs(), wraps + 1)
    } else {
        (dial + times, wraps)
    }
}

#[test]
fn rotate_right_test() {
    assert_eq!(rotate_right(48, 52, 0), (0, 1));
    assert_eq!(rotate_right(50, 1000, 0), (50, 10));
}

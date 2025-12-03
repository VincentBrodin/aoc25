fn main() {
    let input = include_str!("../input_test.txt");
    println!("{}", part1(&input.to_string()));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let mut big_a = 0;
        let mut start_index = 0;
        let chars: Vec<_> = line.chars().collect();
        chars.iter().enumerate().for_each(|(i, c)| {
            if i == chars.len() - 1 {
                return;
            }
            let num = c.to_digit(10).unwrap();
            if big_a < num {
                big_a = num;
                start_index = i;
            }
        });

        let mut big_b = 0;
        chars.iter().skip(start_index + 1).for_each(|c| {
            let num = c.to_digit(10).unwrap();
            if big_b < num {
                big_b = num;
            }
        });
        let mut num_str = String::new();
        num_str.push_str(&big_a.to_string());
        num_str.push_str(&big_b.to_string());
        sum += num_str.parse::<i32>().unwrap();
    }
    sum
}

fn part2(input: &str) -> i32 {
    let lines = input.lines();
    dbg!(&lines);

    let mut sum = 0;
    for line in lines {
        dbg!(line);
        let mut biggest: Vec<_> = vec![];
        let mut start_index = 0;
        let mut left = 12;
        let chars: Vec<_> = line.chars().collect();
        for _ in 0..12 {
            println!("We start on {start_index} with {left} of {}", chars.len());
            let mut big: u32 = 0;
            chars
                .iter()
                .skip(start_index)
                .enumerate()
                .for_each(|(i, c)| {
                    if i + left == chars.len() {
                        return;
                    }

                    let num = c.to_digit(10).unwrap();
                    if big < num {
                        big = num;
                        start_index = i + 1;
                    }
                });
            biggest.push(big);
            left -= 1;
        }
        let mut num_str = String::new();
        biggest
            .iter()
            .for_each(|num| num_str.push_str(&num.to_string()));
        sum += num_str.parse::<i32>().unwrap();
    }
    sum
}

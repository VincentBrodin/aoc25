fn main() {
    let input = include_str!("../input.txt");
    let input = input.replace('\n', "");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let ranges = input.split(',');
    let mut invalids: Vec<_> = vec![];
    ranges.for_each(|range_str| {
        let mut range_split = range_str.split('-');
        let first: i64 = range_split.next().unwrap().parse().unwrap();
        let last: i64 = range_split.next().unwrap().parse().unwrap();
        (first..last + 1).for_each(|i| {
            let i_str = i.to_string();
            if i_str.len() % 2 != 0 {
                return;
            }
            let (left, right) = i_str.split_at(i_str.len() / 2);
            if left == right {
                invalids.push(i);
            }
        });
    });

    invalids.iter().sum()
}

fn part2(input: &str) -> i64 {
    let ranges = input.split(',');
    let mut invalids: Vec<_> = vec![];
    ranges.for_each(|range_str| {
        let mut range_split = range_str.split('-');
        let first: i64 = range_split.next().unwrap().parse().unwrap();
        let last: i64 = range_split.next().unwrap().parse().unwrap();
        (first..last + 1).for_each(|i| {
            let i_str = i.to_string();
            if is_sequence(&i_str) {
                invalids.push(i);
            }
        });
    });

    invalids.iter().sum()
}

fn is_sequence(str: &str) -> bool {
    for i in 1..str.len() {
        let seq = &str[0..i];
        if (i..str.len()).step_by(i).all(|j| {
            if j + i > str.len() {
                return false;
            }
            let chunk = &str[j..j + i];
            seq == chunk
        }) {
            return true;
        }
    }
    false
}

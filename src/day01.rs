pub(crate) fn puzzle_a(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut count = 0;
    for (i, number) in numbers.iter().enumerate().skip(1) {
        if numbers[i - 1] < *number {
            count += 1;
        }
    }
    count
}

pub(crate) fn puzzle_b(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut count = 0;
    for i in 3..numbers.len() {
        if numbers[i - 3] < numbers[i] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_01_a() {
        assert_eq!(puzzle_a(INPUT), 7)
    }

    #[test]
    fn test_01_b() {
        assert_eq!(puzzle_b(INPUT), 5)
    }
}

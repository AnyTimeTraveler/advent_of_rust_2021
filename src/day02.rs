pub(crate) fn puzzle_a(input: &str) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: i32 = amount.parse().unwrap();
        match direction {
            "forward" => pos += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            unexpected => println!("Unexpected input: {}", unexpected)
        }
    }
    depth * pos
}

pub(crate) fn puzzle_b(input: &str) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: i32 = amount.parse().unwrap();
        match direction {
            "forward" => {
                pos += amount;
                depth += aim * amount
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            unexpected => println!("Unexpected input: {}", unexpected)
        }
    }
    depth * pos
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_01_a() {
        assert_eq!(puzzle_a(INPUT), 150)
    }

    #[test]
    fn test_01_b() {
        assert_eq!(puzzle_b(INPUT), 900)
    }
}

fn main() {
    let input = include_str!("./input2.txt");
    let output = parse(input);

    dbg!(output);
}

fn parse(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(|line| process_line(line))
        .sum::<u32>();
    
    output.to_string()
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];

        let result = if reduced_line.starts_with("one") {
            index += "one".len();
            Some('1')
        } else if reduced_line.starts_with("two") {
            index += "two".len();
            Some('2')
        } else if reduced_line.starts_with("three") {
            index += "three".len();
            Some('3')
        } else if reduced_line.starts_with("four") {
            index += "four".len();
            Some('4')
        } else if reduced_line.starts_with("five") {
            index += "five".len();
            Some('5')
        } else if reduced_line.starts_with("six") {
            index += "six".len();
            Some('6')
        } else if reduced_line.starts_with("seven") {
            index += "seven".len();
            Some('7')
        } else if reduced_line.starts_with("eight") {
            index += "eight".len();
            Some('8')
        } else if reduced_line.starts_with("nine") {
            index += "nine".len();
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            index += 1;
            result
        };

        result
    });

    let mut it = line_iter
        .filter_map(|c| c.to_digit(10));

    let first = it.next().expect("Should be a number");

    match it.last() {
        None => format!("{first}{first}"),
        Some(num) => format!("{first}{num}")
    }
        .parse::<u32>()
        .expect("Should be a valid number")
}

mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!("281", parse(input))
    }
}

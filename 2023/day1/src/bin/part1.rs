fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);

    dbg!(output);
}

fn parse(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(|line| {
            let mut it = line
                .chars()
                .filter_map(|c| {
                    c.to_digit(10)
                });

            let first = it.next().expect("Should be a number");
            let last = it.last();

            match last {
                None => format!("{first}{first}").parse::<u32>(),
                Some(num) => format!("{first}{num}").parse::<u32>()
            }.expect("Should be a valid number")
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!("142", parse(input));
    }
}
use rayon::{iter::ParallelIterator, str::ParallelString};

const DIGITS: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn concatenate_ints(a: u32, b: u32) -> u32 {
    let mut multiplier = 1;
    let mut temp = b;

    while temp != 0 {
        multiplier *= 10;
        temp /= 10;
    }

    a * multiplier + b
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<u32> {
    let mut hashmap = hashbrown::HashMap::new();
    let mut rev_hashmap = hashbrown::HashMap::new();

    for i in 0..9 {
        let (word, value) = DIGITS[i];
        let len = word.bytes().len();

        for i in 0..len {
            let value = if &word[0..=i] == word { value } else { 0 };
            hashmap.insert(word[0..=i].as_bytes(), value);
        }

        for i in 0..len {
            let value = if &word[i..len] == word { value } else { 0 };
            rev_hashmap.insert(word[i..len].as_bytes(), value);
        }
    }

    let result = input
        .par_lines()
        .map(|line| {
            let line = line.trim().as_bytes();

            if line.is_empty() {
                return 0;
            }

            let mut left = 0;
            let mut right = line.len() - 1;

            let mut found_left = None;
            let mut found_right = None;

            let mut word_left_pointer = left;
            let mut word_right_pointer = right;

            while right >= left {
                if found_left.is_none() {
                    if line[left].is_ascii_digit() {
                        found_left = Some(line[left] - 48);
                    } else {
                        let lookup = &line[word_left_pointer..=left];
                        let value = hashmap.get(lookup);

                        if let Some(value) = value {
                            if value != &0 {
                                found_left = Some(value.to_owned());
                            }
                        } else {
                            left = word_left_pointer;
                            word_left_pointer += 1;
                        }

                        left += 1;
                    }
                }

                if found_right.is_none() {
                    if line[right].is_ascii_digit() {
                        found_right = Some(line[right] - 48);
                    } else {
                        let lookup = &line[right..=word_right_pointer];
                        let value = rev_hashmap.get(lookup);

                        if let Some(value) = value {
                            if value != &0 {
                                found_right = Some(value.to_owned());
                            }
                        } else {
                            right = word_right_pointer;
                            word_right_pointer -= 1;
                        }

                        right -= 1;
                    }
                }

                if found_right.is_some() && found_left.is_some() {
                    break;
                }
            }
            concatenate_ints(found_left.unwrap() as u32, found_right.unwrap() as u32)
        })
        .reduce(|| 0, |prev, curr| prev + curr);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_base() -> anyhow::Result<()> {
        let input = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        "#;

        assert_eq!(281, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input2.txt");

        assert_eq!(54418, process(input)?);
        Ok(())
    }
}

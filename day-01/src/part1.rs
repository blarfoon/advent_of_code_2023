use rayon::{iter::ParallelIterator, str::ParallelString};
use tracing::info;

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
    let result = input
        .par_lines()
        .map(|line| {
            info!("{}", line);
            let line = line.trim().as_bytes();

            if line.is_empty() {
                return 0;
            }

            let mut left = 0;
            let mut right = line.len() - 1;

            let mut found_left = None;
            let mut found_right = None;

            while right >= left {
                info!(
                    "{} LEFT: {} - POS: {}",
                    line[left],
                    char::from(line[left]),
                    left
                );
                info!(
                    "{} RIGHT: {} - POS: {}",
                    line[right],
                    char::from(line[right]),
                    right
                );
                if line[left].is_ascii_digit() {
                    found_left = Some(line[left] - 48);
                } else {
                    left += 1;
                }

                if line[right].is_ascii_digit() {
                    found_right = Some(line[right] - 48);
                } else {
                    right -= 1;
                }

                if found_right.is_some() && found_left.is_some() {
                    break;
                }
            }
            let res = concatenate_ints(found_left.unwrap() as u32, found_right.unwrap() as u32);
            info!(
                "RES: {} + {} = {}",
                found_left.unwrap(),
                found_right.unwrap(),
                res
            );

            res
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
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#;

        assert_eq!(142, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input1.txt");

        assert_eq!(54304, process(input)?);
        Ok(())
    }
}

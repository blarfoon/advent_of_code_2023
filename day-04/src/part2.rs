fn concatenate_ints(a: u8, b: u8) -> usize {
    if a == 32 {
        return (b - 48) as usize;
    }

    ((a - 48) * 10 + (b - 48)) as usize
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<usize> {
    let mut lines = input.trim().lines().peekable();
    let mut total_copies: usize = 0;

    let (start_row, start_values) = lines
        .peek()
        .map(|&line| {
            let mut start_row = 0;
            let mut start_values = 0;

            for (i, c) in line.chars().enumerate() {
                if c == ':' {
                    start_row = i;
                } else if c == '|' {
                    start_values = i;
                }

                if start_row != 0 && start_values != 0 {
                    break;
                }
            }

            (start_row, start_values)
        })
        .unwrap();

    let mut copies_lookup: [usize; 220] = [0; 220];

    for (line_i, _line) in lines.enumerate() {
        let mut matches: usize = 0;

        let line = _line[start_row + 2..=start_values - 2].as_bytes();

        let mut lookup: [u8; 100] = [0; 100];

        let mut i = 0;
        while i < line.len() {
            let part = &line[i..=i + 1];
            lookup[concatenate_ints(part[0], part[1])] = 1;
            i += 3;
        }

        let line = _line[start_values + 2..].as_bytes();

        let mut i = 0;
        while i < line.len() {
            let part = &line[i..=i + 1];

            let concatenated = concatenate_ints(part[0], part[1]);
            let lookup_res = lookup[concatenated];

            if lookup_res == 1 {
                matches += 1;
            }
            i += 3;
        }

        if matches > 0 {
            for i in line_i + 1..line_i + 1 + matches {
                copies_lookup[i] += 1 + copies_lookup[line_i];
            }
        }

        total_copies += 1 + copies_lookup[line_i];
    }

    Ok(total_copies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input2_small.txt");

        assert_eq!(30, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input2.txt");

        assert_eq!(5920640, process(input)?);
        Ok(())
    }
}

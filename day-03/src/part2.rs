struct Number {
    start: usize,
    end: usize,
    number: usize,
}

fn read_number_bidirectional(line: &[u8], initial: usize) -> Number {
    let mut start = initial;
    let mut end = initial;

    loop {
        let mut should_break = true;
        if start > 0 && line[start - 1].is_ascii_digit() {
            start -= 1;
            should_break = false;
        }

        if end < line.len() - 1 && line[end + 1].is_ascii_digit() {
            end += 1;
            should_break = false;
        }

        if should_break {
            break;
        }
    }

    Number {
        start,
        end,
        number: std::str::from_utf8(&line[start..=end])
            .unwrap()
            .parse()
            .unwrap(),
    }
}

fn scan_str_for_numbers(line: &[u8], start: usize, end: usize) -> [Option<usize>; 2] {
    let mut current = start;
    let mut numbers: [Option<usize>; 2] = [None, None];

    while current <= end {
        if line[current].is_ascii_digit() {
            let res = read_number_bidirectional(line, current);
            current = res.end + 1;
            if numbers[0].is_none() {
                numbers[0] = Some(res.number);
            } else {
                numbers[1] = Some(res.number);
            }
        } else {
            current += 1;
        }
    }

    numbers
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<usize> {
    let mut line_counter = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let lines_len = lines.len();
    let mut sum = 0;
    while line_counter < lines_len {
        let line = lines[line_counter].as_bytes();

        let mut char_counter = 0;

        while char_counter < line.len() {
            if line[char_counter] == b'*' {
                let mut gear_ratio = 1;
                let mut numbers_count = 0;

                if line_counter > 0 {
                    let prev_line = lines[line_counter - 1].as_bytes();

                    let start = if char_counter > 0 {
                        char_counter - 1
                    } else {
                        char_counter
                    };
                    let end = if char_counter < prev_line.len() {
                        char_counter + 1
                    } else {
                        char_counter
                    };

                    let numbers = scan_str_for_numbers(prev_line, start, end);

                    if let Some(number) = numbers[0] {
                        numbers_count += 1;
                        if numbers_count > 2 {
                            char_counter += 1;
                            continue;
                        }
                        gear_ratio *= number;
                    }

                    if let Some(number) = numbers[1] {
                        numbers_count += 1;
                        if numbers_count > 2 {
                            char_counter += 1;
                            continue;
                        }
                        gear_ratio *= number;
                    }
                }

                if line_counter < line.len() {
                    let next_line = lines[line_counter + 1].as_bytes();

                    let start = if char_counter > 0 {
                        char_counter - 1
                    } else {
                        char_counter
                    };
                    let end = if char_counter < next_line.len() {
                        char_counter + 1
                    } else {
                        char_counter
                    };

                    let numbers = scan_str_for_numbers(next_line, start, end);

                    if let Some(number) = numbers[0] {
                        numbers_count += 1;
                        if numbers_count > 2 {
                            char_counter += 1;
                            continue;
                        }
                        gear_ratio *= number;
                    }

                    if let Some(number) = numbers[1] {
                        numbers_count += 1;
                        if numbers_count > 2 {
                            char_counter += 1;
                            continue;
                        }
                        gear_ratio *= number;
                    }
                }

                if char_counter > 0 {
                    let prev_char = line[char_counter - 1];
                    if prev_char.is_ascii_digit() {
                        let number = read_number_bidirectional(line, char_counter - 1);
                        numbers_count += 1;
                        if numbers_count > 2 {
                            char_counter += 1;
                            continue;
                        }
                        gear_ratio *= number.number;
                    }
                }

                if char_counter < line.len() {
                    let next_char = line[char_counter + 1];
                    if next_char.is_ascii_digit() {
                        let number = read_number_bidirectional(line, char_counter + 1);
                        numbers_count += 1;
                        if numbers_count > 2 {
                            char_counter += 1;
                            continue;
                        }
                        gear_ratio *= number.number;
                    }
                }

                if numbers_count != 2 {
                    char_counter += 1;
                    continue;
                } else {
                    sum += gear_ratio;
                }
            }

            char_counter += 1;
        }

        line_counter += 1;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input2_small.txt");

        assert_eq!(467835, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input2.txt");

        assert_eq!(87263515, process(input)?);
        Ok(())
    }
}

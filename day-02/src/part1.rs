const RED_CUBES: usize = 12;
const GREEN_CUBES: usize = 13;
const BLUE_CUBES: usize = 14;

const GAME_SKIP_CHARS: usize = 2; // to first number

const GREEN_SKIP_CHARS: usize = 7;
const BLUE_SKIP_CHARS: usize = 6;
const RED_SKIP_CHARS: usize = 5;

fn read_while_number(line: &str, arr: &[u8], initial: usize) -> (usize, usize) {
    if initial > line.len() {
        return (0, 0);
    }

    let mut i = 0;
    while arr[initial + i].is_ascii_digit() {
        i += 1;
    }

    (line[initial..initial + i].parse().unwrap(), i - 1)
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;
    for (line_i, line) in input.trim().lines().enumerate() {
        let chars = line.as_bytes();
        let len = chars.len();

        let mut current_number = 0;

        let mut i = 0;
        let mut is_valid = true;

        while i < len && is_valid {
            let c = chars[i];
            match c {
                b':' => {
                    i += GAME_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                b'r' => {
                    if current_number > RED_CUBES {
                        is_valid = false;
                        break;
                    }
                    i += RED_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                b'g' => {
                    if current_number > GREEN_CUBES {
                        is_valid = false;
                        break;
                    }
                    i += GREEN_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                b'b' => {
                    if current_number > BLUE_CUBES {
                        is_valid = false;
                        break;
                    }
                    i += BLUE_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                _ => i += 1,
            }
        }

        if is_valid {
            sum += line_i + 1;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input1_small.txt");

        assert_eq!(8, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input1.txt");

        assert_eq!(2720, process(input)?);
        Ok(())
    }
}

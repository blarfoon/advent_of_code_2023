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
    for line in input.trim().lines() {
        let chars = line.as_bytes();
        let len = chars.len();

        let mut current_number = 0;

        let mut i = 0;

        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;

        while i < len {
            let c = chars[i];
            match c {
                b':' => {
                    i += GAME_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                b'r' => {
                    if current_number > max_red {
                        max_red = current_number;
                    }
                    i += RED_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                b'g' => {
                    if current_number > max_green {
                        max_green = current_number;
                    }
                    i += GREEN_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                b'b' => {
                    if current_number > max_blue {
                        max_blue = current_number;
                    }
                    i += BLUE_SKIP_CHARS;
                    let (n, l) = read_while_number(line, chars, i);
                    current_number = n;
                    i += l + 2;
                }
                _ => i += 1,
            }
        }

        sum += max_blue * max_green * max_red
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

        assert_eq!(2286, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input2.txt");

        assert_eq!(0, process(input)?);
        Ok(())
    }
}

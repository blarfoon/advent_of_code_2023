fn read_while_number(line_bytes: &[u8], line: &str, initial_position: usize) -> (usize, usize) {
    if initial_position > line.len() {
        return (0, 0);
    }

    let mut i = 0;

    while let Some(c) = line_bytes.get(initial_position + i) {
        if c.is_ascii_digit() {
            i += 1;
        } else {
            break;
        }
    }

    (
        line[initial_position..initial_position + i]
            .parse::<usize>()
            .unwrap(),
        i,
    )
}

fn is_symbol(val: u8) -> bool {
    val != b'.' && !val.is_ascii_digit()
}

fn has_adj_symbol(line: &[u8], start: usize, end: usize) -> bool {
    if line.is_empty() {
        return false;
    }

    for &b in &line[start..=end] {
        if is_symbol(b) {
            return true;
        }
    }

    false
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;

    let mut line_i = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let lines_len = lines.len();

    while line_i < lines_len {
        let prev_line = lines
            .get(if line_i == 0 {
                lines_len + 1
            } else {
                line_i - 1
            })
            .unwrap_or(&"")
            .trim();
        let prev_line_bytes = prev_line.as_bytes();
        let line = lines[line_i].trim();
        let line_bytes = line.as_bytes();
        let next_line = lines.get(line_i + 1).unwrap_or(&"").trim();
        let next_line_bytes = next_line.as_bytes();

        let mut i = 0;
        while i < line_bytes.len() {
            let c = line_bytes[i];
            if c.is_ascii_digit() {
                let prev_char = line_bytes[i.saturating_sub(1)];
                let prev_i = i.saturating_sub(1);
                let (n, c) = read_while_number(line_bytes, line, i);
                i += c;
                let next_char = line_bytes[if i == line_bytes.len() { i - 1 } else { i }];

                let end = if i >= line_bytes.len() {
                    line_bytes.len() - 1
                } else {
                    i
                };

                if is_symbol(prev_char)
                    || is_symbol(next_char)
                    || has_adj_symbol(prev_line_bytes, prev_i, end)
                    || has_adj_symbol(next_line_bytes, prev_i, end)
                {
                    sum += n;
                }
            }

            i += 1;
        }

        line_i += 1;
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

        assert_eq!(4361, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input1.txt");

        assert_eq!(554003, process(input)?);
        Ok(())
    }
}

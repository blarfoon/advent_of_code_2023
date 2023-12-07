use std::time::Duration;

fn parse_line_number(line: &str) -> usize {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

#[tracing::instrument]
pub fn process(_input: &str) -> anyhow::Result<usize> {
    // let time = std::time::SystemTime::now()
    //     .duration_since(std::time::SystemTime::UNIX_EPOCH)
    //     .unwrap()
    //     .as_micros();

    let mut lines = _input.lines();

    let allowed_time = parse_line_number(lines.next().unwrap());
    let record_distance = parse_line_number(lines.next().unwrap());

    let mut _possible_ways: usize = 0;

    let mut pressing_time = 1;
    let mut run_time = allowed_time - 1;

    loop {
        let race_time = pressing_time * run_time;
        if race_time > record_distance {
            _possible_ways += 1;
        }

        pressing_time += 1;
        run_time -= 1;

        if run_time == 0 {
            break;
        }
    }

    // let new_time = std::time::SystemTime::now()
    //     .duration_since(std::time::SystemTime::UNIX_EPOCH)
    //     .unwrap()
    //     .as_micros();

    // println!("took {}micro", new_time - time);

    Ok(_possible_ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input2_small.txt");

        assert_eq!(71503, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input2.txt");

        assert_eq!(32583852, process(input)?);
        Ok(())
    }
}

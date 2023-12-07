#[tracing::instrument]
pub fn process(_input: &str) -> anyhow::Result<usize> {
    let mut races: [(usize, usize); 10] = [(0, 0); 10];

    for (i, line) in _input.lines().enumerate() {
        let mut curr_number_i = 0;

        let mut number_start = None;
        for (mut c_i, c) in line.char_indices() {
            if c.is_ascii_digit() && c_i != line.len() - 1 {
                if number_start.is_none() {
                    number_start = Some(c_i);
                }
            } else if let Some(mut _number_start) = number_start {
                number_start = None;

                if c_i == line.len() - 1 {
                    c_i += 1;
                }

                if i == 0 {
                    races[curr_number_i].0 = line[_number_start..c_i].parse().unwrap()
                } else if i == 1 {
                    races[curr_number_i].1 = line[_number_start..c_i].parse().unwrap()
                }

                curr_number_i += 1;
            }
        }
    }

    Ok(races
        .iter()
        .filter(|v| v.0 != 0 && v.1 != 0)
        .map(|(allowed_time, record_distance)| {
            let mut _possible_ways: usize = 0;

            let mut pressing_time = 1;
            let mut run_time = allowed_time - 1;

            loop {
                let race_time = pressing_time * run_time;
                if race_time > *record_distance {
                    _possible_ways += 1;
                }

                pressing_time += 1;
                run_time -= 1;

                if run_time == 0 || (pressing_time > run_time && race_time < *record_distance) {
                    break;
                }
            }

            _possible_ways
        })
        .reduce(|acc, curr| acc * curr)
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input1_small.txt");

        assert_eq!(288, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input1.txt");

        assert_eq!(1624896, process(input)?);
        Ok(())
    }
}

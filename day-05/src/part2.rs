use std::usize;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Clone, Copy, Debug)]
struct ItemMap {
    source_start: usize,
    source_end: usize,
    destination_start: usize,
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<usize> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut sources = Vec::new();

    let first_line = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut counter = 0;
    for x in first_line.chunks_exact(2) {
        let start = x[0];
        let length = x[1];

        sources.resize(sources.len() + length, 0);

        for i in 0..length {
            sources[counter] = start + i;
            counter += 1;
        }
    }

    let mut category_section_id_start = 3;

    while category_section_id_start < lines.len() {
        let mut mappings: [Option<ItemMap>; 50] = [None; 50];
        let mut category_section_id_end = category_section_id_start;

        while let Some(line) = lines.get(category_section_id_end) {
            if let Some(number) = line.as_bytes().first() {
                if !number.is_ascii_digit() {
                    break;
                }

                let mut numbers = line
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<usize>().unwrap());

                let destination_start = numbers.next().unwrap();
                let source_start = numbers.next().unwrap();
                let length = numbers.next().unwrap();

                mappings[category_section_id_end - category_section_id_start] = Some(ItemMap {
                    source_start,
                    source_end: source_start + length - 1,
                    destination_start,
                });

                category_section_id_end += 1;
            } else {
                break;
            }
        }

        sources
            .par_iter_mut()
            .filter(|v| **v != 0)
            .for_each(|source| {
                if *source == 0 {
                    return;
                }

                for mapping in mappings {
                    let Some(mapping) = mapping else {
                        break;
                    };

                    if *source >= mapping.source_start && *source <= mapping.source_end {
                        *source = *source - mapping.source_start + mapping.destination_start;
                        break;
                    }
                }
            });

        category_section_id_start = category_section_id_end + 2;
    }

    Ok(*sources
        .iter()
        .reduce(|acc, curr| if curr < acc && curr != &0 { curr } else { acc })
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input2_small.txt");

        assert_eq!(46, process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input2.txt");

        assert_eq!(10834440, process(input)?);
        Ok(())
    }
}

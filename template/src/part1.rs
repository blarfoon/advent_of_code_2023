#[tracing::instrument]
pub fn process(_input: &str) -> anyhow::Result<String> {
    todo!("day {{project-name}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_process_small() -> anyhow::Result<()> {
        let input = include_str!("../input1_small.txt");

        assert_eq!("", process(input)?);
        Ok(())
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_process() -> anyhow::Result<()> {
        let input = include_str!("../input1.txt");

        assert_eq!("", process(input)?);
        Ok(())
    }
}

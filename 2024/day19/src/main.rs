use eyre::{bail, OptionExt, Report};
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() -> Result<(), Report> {
    let (towels, designs) = parse(stdin().lock())?;
    let possible_design_count = count_possible_designs(&towels, &designs);
    println!("{} designs are possible.", possible_design_count);

    Ok(())
}

fn parse(input: impl BufRead) -> Result<(Vec<String>, Vec<String>), Report> {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .ok_or_eyre("Empty input")??
        .split(", ")
        .map(|s| s.to_owned())
        .collect();

    if !lines.next().ok_or_eyre("Missing blank line")??.is_empty() {
        bail!("Second line is not blank");
    }
    let designs = lines.collect::<Result<_, _>>()?;

    Ok((towels, designs))
}

fn count_possible_designs(towels: &[String], designs: &[String]) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|design| is_design_possible(&towels, design, &mut cache))
        .count()
}

/// Returns whether it is possible to make the given design from the given towels.
fn is_design_possible(towels: &[String], design: &str, cache: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    } else if let Some(possible) = cache.get(design) {
        return *possible;
    }
    for towel in towels {
        if let Some(remainder) = design.strip_prefix(towel) {
            if is_design_possible(towels, remainder, cache) {
                cache.insert(design.to_owned(), true);
                return true;
            }
        }
    }
    cache.insert(design.to_owned(), false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_example() {
        let (towels, designs) = parse(
            "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            towels,
            vec![
                "r".to_string(),
                "wr".to_string(),
                "b".to_string(),
                "g".to_string(),
                "bwu".to_string(),
                "rb".to_string(),
                "gb".to_string(),
                "br".to_string(),
            ]
        );
        assert_eq!(
            designs,
            vec![
                "brwrr".to_string(),
                "bggr".to_string(),
                "gbbr".to_string(),
                "rrbgbr".to_string(),
                "ubwu".to_string(),
                "bwurrg".to_string(),
                "brgr".to_string(),
                "bbrgwb".to_string(),
            ]
        );
        assert_eq!(count_possible_designs(&towels, &designs), 6);
    }
}

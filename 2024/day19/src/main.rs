use eyre::{bail, OptionExt, Report};
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn main() -> Result<(), Report> {
    let (towels, designs) = parse(stdin().lock())?;
    let all_arrangements = count_all_arrangements(&towels, &designs);
    let possible_design_count = all_arrangements.iter().filter(|count| **count > 0).count();
    println!("{} designs are possible.", possible_design_count);
    println!(
        "{} different arrangements are possible across all designs.",
        all_arrangements.iter().sum::<usize>()
    );

    Ok(())
}

fn parse(input: impl BufRead) -> Result<(HashSet<String>, Vec<String>), Report> {
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

/// Returns how many different ways each design is possible.
fn count_all_arrangements(towels: &HashSet<String>, designs: &[String]) -> Vec<usize> {
    let Some(max_towel_size) = towels.iter().map(|towel| towel.len()).max() else {
        return vec![0; designs.len()];
    };
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| count_possible_arrangements(towels, max_towel_size, design, &mut cache))
        .collect()
}

/// Returns the number of ways it is possible to make the given design from the given towels.
fn count_possible_arrangements(
    towels: &HashSet<String>,
    max_towel_size: usize,
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    } else if let Some(possible) = cache.get(design) {
        return *possible;
    }
    let possible_arrangement_count = (1..=min(max_towel_size, design.len()))
        .map(|prefix_len| {
            if towels.contains(&design[..prefix_len]) {
                count_possible_arrangements(towels, max_towel_size, &design[prefix_len..], cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(design.to_owned(), possible_arrangement_count);
    possible_arrangement_count
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
            [
                "r".to_string(),
                "wr".to_string(),
                "b".to_string(),
                "g".to_string(),
                "bwu".to_string(),
                "rb".to_string(),
                "gb".to_string(),
                "br".to_string(),
            ]
            .into_iter()
            .collect()
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
        assert_eq!(
            count_all_arrangements(&towels, &designs),
            vec![2, 1, 4, 6, 0, 1, 2, 0]
        );
    }
}

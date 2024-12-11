use eyre::{OptionExt, Report};
use std::{
    cmp::Ordering,
    collections::HashSet,
    io::{stdin, BufRead},
};

fn main() -> Result<(), Report> {
    let (rules, updates) = parse(stdin().lock())?;
    let correct_middle_sum = sum_correct_middle_pages(&rules, &updates);
    let incorrect_middle_sum = sort_sum_incorrect(&rules, &updates);
    println!(
        "Sum of middle pages from correct updates: {}",
        correct_middle_sum
    );
    println!(
        "Sum of middle pages from incorrect updates after sorting: {}",
        incorrect_middle_sum
    );

    Ok(())
}

fn parse(input: impl BufRead) -> Result<(Vec<(u64, u64)>, Vec<Vec<u64>>), Report> {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (before, after) = line.split_once('|').ok_or_eyre("Missing '|'")?;
        rules.push((before.parse()?, after.parse()?));
    }

    let updates = lines
        .map(|line| {
            let line = line?;
            line.split(',').map(|page| Ok(page.parse()?)).collect()
        })
        .collect::<Result<_, Report>>()?;

    Ok((rules, updates))
}

/// Filters the updates to those which are correct according to the given ordering rules, then sums
/// their middle page numbers.
fn sum_correct_middle_pages(rules: &[(u64, u64)], updates: &[Vec<u64>]) -> u64 {
    updates
        .iter()
        .filter(|update| is_order_correct(rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

/// Returns whether this update is in the correct order according to the given ordering rules.
fn is_order_correct(rules: &[(u64, u64)], update: &[u64]) -> bool {
    rules.iter().all(|(before, after)| {
        let Some(before_index) = update.iter().position(|u| u == before) else {
            return true;
        };
        let Some(after_index) = update.iter().position(|u| u == after) else {
            return true;
        };
        before_index <= after_index
    })
}

/// Filters the updates to those which are not correct, sorts them to be correct, then sums their
/// middle page numbers.
fn sort_sum_incorrect(rules: &[(u64, u64)], updates: &[Vec<u64>]) -> u64 {
    updates
        .iter()
        .filter(|update| !is_order_correct(rules, update))
        .map(|update| sort(rules, update)[update.len() / 2])
        .sum()
}

/// Sorts the given update according to the given rules.
fn sort(rules: &[(u64, u64)], update: &[u64]) -> Vec<u64> {
    let rules: HashSet<(u64, u64)> = rules.iter().copied().collect();
    let mut new = update.to_owned();
    new.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_RULES: &[(u64, u64)] = &[
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];

    #[test]
    fn parse_empty() {
        assert_eq!(parse("".as_bytes()).unwrap(), (vec![], vec![]));
    }

    #[test]
    fn parse_small() {
        assert_eq!(
            parse("12|43\n42|66\n\n1,2,3\n4,5\n6".as_bytes()).unwrap(),
            (
                vec![(12, 43), (42, 66)],
                vec![vec![1, 2, 3], vec![4, 5], vec![6]]
            )
        );
    }

    #[test]
    fn correct_orderings() {
        assert!(is_order_correct(EXAMPLE_RULES, &[75, 47, 61, 53, 29]));
        assert!(is_order_correct(EXAMPLE_RULES, &[97, 61, 53, 29, 13]));
        assert!(is_order_correct(EXAMPLE_RULES, &[75, 29, 13]));
        assert!(!is_order_correct(EXAMPLE_RULES, &[75, 97, 47, 61, 53]));
        assert!(!is_order_correct(EXAMPLE_RULES, &[61, 13, 29]));
        assert!(!is_order_correct(EXAMPLE_RULES, &[97, 13, 75, 29, 47]));
    }

    #[test]
    fn sum_correct_example() {
        assert_eq!(
            sum_correct_middle_pages(
                EXAMPLE_RULES,
                &[
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47],
                ]
            ),
            143
        );
    }

    #[test]
    fn sort_example() {
        assert_eq!(
            sort(EXAMPLE_RULES, &[75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(sort(EXAMPLE_RULES, &[61, 13, 29]), vec![61, 29, 13]);
        assert_eq!(
            sort(EXAMPLE_RULES, &[97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn sum_incorrect_example() {
        assert_eq!(
            sort_sum_incorrect(
                EXAMPLE_RULES,
                &[
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47],
                ]
            ),
            123
        );
    }
}

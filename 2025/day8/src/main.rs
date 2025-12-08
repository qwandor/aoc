use eyre::{Report, eyre};
use std::{
    collections::BTreeMap,
    io::{BufRead, stdin},
};

fn main() -> Result<(), Report> {
    let positions = parse(stdin().lock())?;

    println!(
        "Three largest sizes multiplied after 1000 connections: {}",
        connect_count_multiply(&positions, 1000)
    );

    Ok(())
}

fn parse(input: impl BufRead) -> Result<Vec<[i64; 3]>, Report> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            line.split(',')
                .map(|part| part.parse())
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|v: Vec<i64>| eyre!("{} numbers on line, expected 3", v.len()))
        })
        .collect()
}

/// Make the `connection_count` shortest connections, then count how many junction boxes are in the
/// three biggest circuits and multiply them.
fn connect_count_multiply(positions: &[[i64; 3]], connection_count: usize) -> u32 {
    // Find the closest `connection_count` pairs of junction boxes.
    let mut distances = Vec::new();
    for (i, a) in positions.iter().enumerate() {
        for (j, b) in positions[0..i].iter().enumerate() {
            if i != j {
                distances.push((i, j, distance_squared(a, b)));
            }
        }
    }
    distances.sort_by_key(|&(_, _, distance)| distance);
    println!(
        "Closest {connection_count}: {:?}",
        &distances[0..connection_count]
    );

    // Connect them, and count the sizes of the resulting subgraphs.
    let mut subgraphs = (0..positions.len()).collect::<Vec<_>>();
    for &(i, j, _) in &distances[0..connection_count] {
        let old = subgraphs[i];
        let new = subgraphs[j];
        for entry in &mut subgraphs {
            if *entry == old {
                *entry = new;
            }
        }
    }

    let mut counts = BTreeMap::<_, u32>::new();
    for &subgraph in &subgraphs {
        *counts.entry(subgraph).or_default() += 1;
    }
    let mut counts_values = counts.values().copied().collect::<Vec<_>>();
    counts_values.sort();
    counts_values.reverse();

    counts_values[0..3].iter().product()
}

fn distance_squared(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    a.iter()
        .zip(b.iter())
        .map(|(&a, &b)| (a - b) * (a - b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
                .as_bytes(),
            )
            .unwrap(),
            vec![
                [162, 817, 812],
                [57, 618, 57],
                [906, 360, 560],
                [592, 479, 940],
                [352, 342, 300],
                [466, 668, 158],
                [542, 29, 236],
                [431, 825, 988],
                [739, 650, 466],
                [52, 470, 668],
                [216, 146, 977],
                [819, 987, 18],
                [117, 168, 530],
                [805, 96, 715],
                [346, 949, 466],
                [970, 615, 88],
                [941, 993, 340],
                [862, 61, 35],
                [984, 92, 344],
                [425, 690, 689],
            ]
        );
    }

    #[test]
    fn example() {
        assert_eq!(
            connect_count_multiply(
                &[
                    [162, 817, 812],
                    [57, 618, 57],
                    [906, 360, 560],
                    [592, 479, 940],
                    [352, 342, 300],
                    [466, 668, 158],
                    [542, 29, 236],
                    [431, 825, 988],
                    [739, 650, 466],
                    [52, 470, 668],
                    [216, 146, 977],
                    [819, 987, 18],
                    [117, 168, 530],
                    [805, 96, 715],
                    [346, 949, 466],
                    [970, 615, 88],
                    [941, 993, 340],
                    [862, 61, 35],
                    [984, 92, 344],
                    [425, 690, 689],
                ],
                10
            ),
            40
        );
    }
}

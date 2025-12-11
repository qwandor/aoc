use eyre::{Report, bail};
use std::{
    collections::BTreeMap,
    io::{BufRead, stdin},
};

fn main() -> Result<(), Report> {
    let connections = parse(stdin().lock())?;

    println!(
        "Paths from you to out: {}",
        count_paths(&connections, "you", "out", &[], &[])
    );
    println!(
        "Paths from svr to out, visiting dac and fft: {}",
        count_paths(&connections, "svr", "out", &["dac", "fft"], &[])
    );

    Ok(())
}

fn parse(input: impl BufRead) -> Result<BTreeMap<String, Vec<String>>, Report> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let Some((device, outputs)) = line.split_once(": ") else {
                bail!("Missing colon");
            };
            let outputs = outputs.split(' ').map(ToOwned::to_owned).collect();
            Ok((device.to_owned(), outputs))
        })
        .collect()
}

fn count_paths(
    connections: &BTreeMap<String, Vec<String>>,
    from: &str,
    to: &str,
    must_visit: &[&str],
    visited: &[&str],
) -> usize {
    if visited.contains(&from) {
        println!("Loop: {visited:?}");
        return 0;
    }
    let visited = visited
        .iter()
        .map(Clone::clone)
        .chain(Some(from))
        .collect::<Vec<_>>();

    if from == to {
        if must_visit
            .iter()
            .all(|must_visit_device| visited.contains(must_visit_device))
        {
            1
        } else {
            0
        }
    } else {
        connections
            .get(from)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|output| count_paths(connections, output, to, must_visit, &visited))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
                .as_bytes(),
            )
            .unwrap(),
            [
                (
                    "aaa".to_string(),
                    vec!["you".to_string(), "hhh".to_string()]
                ),
                (
                    "you".to_string(),
                    vec!["bbb".to_string(), "ccc".to_string()]
                ),
                (
                    "bbb".to_string(),
                    vec!["ddd".to_string(), "eee".to_string()]
                ),
                (
                    "ccc".to_string(),
                    vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()]
                ),
                ("ddd".to_string(), vec!["ggg".to_string()]),
                ("eee".to_string(), vec!["out".to_string()]),
                ("fff".to_string(), vec!["out".to_string()]),
                ("ggg".to_string(), vec!["out".to_string()]),
                (
                    "hhh".to_string(),
                    vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()]
                ),
                ("iii".to_string(), vec!["out".to_string()]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn count_example_paths_you_to_out() {
        assert_eq!(
            count_paths(
                &[
                    (
                        "aaa".to_string(),
                        vec!["you".to_string(), "hhh".to_string()]
                    ),
                    (
                        "you".to_string(),
                        vec!["bbb".to_string(), "ccc".to_string()]
                    ),
                    (
                        "bbb".to_string(),
                        vec!["ddd".to_string(), "eee".to_string()]
                    ),
                    (
                        "ccc".to_string(),
                        vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()]
                    ),
                    ("ddd".to_string(), vec!["ggg".to_string()]),
                    ("eee".to_string(), vec!["out".to_string()]),
                    ("fff".to_string(), vec!["out".to_string()]),
                    ("ggg".to_string(), vec!["out".to_string()]),
                    (
                        "hhh".to_string(),
                        vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()]
                    ),
                    ("iii".to_string(), vec!["out".to_string()]),
                ]
                .into_iter()
                .collect(),
                "you",
                "out",
                &[],
                &[],
            ),
            5
        );
    }

    #[test]
    fn count_example_paths_svr_to_out() {
        assert_eq!(
            count_paths(
                &parse(
                    "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
                    .as_bytes()
                )
                .unwrap(),
                "svr",
                "out",
                &["dac", "fft"],
                &[],
            ),
            2
        );
    }
}

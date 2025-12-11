use eyre::{Report, bail};
use std::{
    collections::BTreeMap,
    io::{BufRead, stdin},
};

fn main() -> Result<(), Report> {
    let connections = parse(stdin().lock())?;

    println!(
        "Paths from you to out: {}",
        count_paths(&connections, "you", &["out"])
    );
    println!(
        "Paths from svr to out, visiting dac and fft: {}",
        count_paths(&connections, "svr", &["dac", "fft", "out"])
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
    must_visit: &[&str],
) -> usize {
    memoise(
        connections,
        (
            from.to_string(),
            must_visit.iter().map(|s| s.to_string()).collect(),
        ),
        count_paths_memoised,
        &mut BTreeMap::new(),
    )
}

fn memoise<T: Copy, I: Clone + Ord, O: Copy>(
    data: T,
    input: I,
    f: fn(T, I, &mut BTreeMap<I, O>) -> O,
    memo: &mut BTreeMap<I, O>,
) -> O {
    if let Some(&answer) = memo.get(&input) {
        answer
    } else {
        let answer = f(data, input.clone(), memo);
        memo.insert(input, answer);
        answer
    }
}

fn count_paths_memoised(
    connections: &BTreeMap<String, Vec<String>>,
    (from, must_visit): (String, Vec<String>),
    memo: &mut BTreeMap<(String, Vec<String>), usize>,
) -> usize {
    let must_visit = must_visit
        .iter()
        .filter(|&device| device != &from)
        .cloned()
        .collect::<Vec<_>>();
    if must_visit.is_empty() {
        1
    } else {
        if let Some(outs) = connections.get(&from) {
            outs.iter()
                .map(|output| {
                    memoise(
                        connections,
                        (output.clone(), must_visit.clone()),
                        count_paths_memoised,
                        memo,
                    )
                })
                .sum()
        } else {
            0
        }
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
                &["out"],
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
                &["dac", "fft", "out"],
            ),
            2
        );
    }
}

use std::io::stdin;

fn main() {
    // Read input from stdin and find total for each elf.
    let mut elf_totals = vec![];
    let mut current_elf_sum: u32 = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elf_totals.push(current_elf_sum);
            current_elf_sum = 0;
        } else {
            current_elf_sum += line.parse::<u32>().unwrap();
        }
    }
    if current_elf_sum > 0 {
        elf_totals.push(current_elf_sum);
    }

    elf_totals.sort_by(|a, b| b.cmp(a));
    println!(
        "{} + {} + {} = {}",
        elf_totals[0],
        elf_totals[1],
        elf_totals[2],
        elf_totals[0] + elf_totals[1] + elf_totals[2],
    );
}

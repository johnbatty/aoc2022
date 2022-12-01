use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Elf {
    id: usize,
    food_calories: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.food_calories.iter().sum()
    }
}

fn parse_input(filename: &str) -> Result<Vec<Elf>> {
    let data = std::fs::read_to_string(filename)?;
    let per_elf_data = data.split("\n\n");

    per_elf_data
        .enumerate()
        .map(|(id, elf_data)| {
            let food_calories = elf_data
                .lines()
                .map(|line| {
                    line.parse::<u32>()
                        .map_err(|e| anyhow!("Failed to parse value: {}", e))
                })
                .collect::<Result<Vec<u32>>>()?;
            Ok(Elf { id, food_calories })
        })
        .collect()
}

fn main() -> Result<()> {
    let mut elves = parse_input("data.txt")?;
    println!("Elves: {:#?}", elves);

    println!("\nCalorie counts:");
    elves.sort_by_key(|elf| std::cmp::Reverse(elf.total_calories()));

    if let Some(top_elf) = elves.first() {
        println!(
            "PART1: Elf {} has the most calories: {}",
            top_elf.id,
            top_elf.total_calories()
        );
    }

    let top3_calories: u32 = elves.iter().take(3).map(|elf| elf.total_calories()).sum();
    println!("PART2: Top 3 elves have {} calories", top3_calories);

    Ok(())
}

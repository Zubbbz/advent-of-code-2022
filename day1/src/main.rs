use std::{fs, path::Path};
const INPUTFILEPATH: &str =
	"/home/nathan/repos/advent-of-code-2022/day1/input.txt";

fn str_to_path(path: &str) -> &Path {
	Path::new(path)
}

fn parse_input(path: &Path) -> Vec<Vec<u32>> {
	let mut elves: Vec<Vec<u32>> = vec![vec![]];
	let mut iter: usize = 0;

	let data = fs::read_to_string(path).expect(
		"Failed to open the input file (did you esure the path was correct?)",
	);

	for line in data.lines() {
		match line.parse() {
			Ok(n) => elves[iter].push(n),
			Err(_err) => {
				iter += 1;
				elves.push(vec![]);
			}
		}
	}

	// return a vector of vectors of u32
	elves
}

fn parse_elf_calories(elves: &[Vec<u32>]) -> Vec<u32> {
	return elves
		.iter()
		.map(|inner_vec| inner_vec
			.iter()
			.sum())
			.collect();
}

fn main() {
	let elves = parse_input(str_to_path(INPUTFILEPATH));
	let mut totals = parse_elf_calories(&elves);
	totals.sort_by(|a, b| a.cmp(b).reverse());
	let top_three = totals[0] + totals[1] + totals[2];
	println!("totals: {:#?}\nNumber 1: {:#?}\n Top3: {:#?}", totals, totals[0], top_three);
}

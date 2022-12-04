pub fn part_one(input: &str) -> Option<u32> {
    let elfs: std::str::Split<&str> = input.split("\n\n");

    let mut max_calories: Option<u32> = Some(0);

    for elf in elfs {
        let calories = elf
        .split('\n')
        .map(|meals| meals.parse::<u32>().unwrap_or(0))
        .reduce(|x, y| x+y);

        if calories > max_calories {
            max_calories = calories;
        }
    }
    max_calories
}

pub fn part_two(input: &str) -> Option<u32> {
    let elfs = input.split("\n\n");

    let mut fat_elves: Vec<u32> = vec![0,0,0];
    
    for elf in elfs{
         let calories = elf
        .split('\n')
        .map(|meals| meals.parse::<u32>().unwrap_or(0))
        .reduce(|x, y| x+y)
        .unwrap_or(0);

        let min_value_index = min_index(&fat_elves);
        if calories > fat_elves[min_value_index] {
            fat_elves[min_value_index] = calories
        }
    }
    Some(fat_elves.iter().sum())
}

fn min_index(input_vec: &Vec<u32>) -> usize{
    let mut i = 0;

    for (j, &amount) in input_vec.iter().enumerate() {
        if amount < input_vec[i]{
            i = j;
        }
    }
    i
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}

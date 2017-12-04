static INPUT: u32 = 368078;

pub fn solution() {
    println!("Day 3");
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
    println!("-----------------");
}

fn get_start_of_outter_ring(input: u32) -> (i32, i32, u32) {
    if input == 1 {
        return (0, 0, 1);
    }

    let mut size = 3;
    let mut x = 1;
    let mut y = 0;
    loop {
        if input <= size * size {
            return (x, y, size);
        }
        x += 1;
        y -= 1;
        size += 2;
    }
}

fn part1(input: u32) -> u32 {
    // Special case for input == 1
    if input == 1 {
        return 0;
    }

    let (mut x, mut y, size) = get_start_of_outter_ring(input);
    let mut start = ((size - 2) * (size - 2)) + 1;

    // This loop is done 1 less time than others because we are starting
    // on the first outter number.
    for _ in 0..size - 2 {
        if start == input {
            return (x.abs() + y.abs()) as u32;
        }
        start += 1;
        y += 1;
    }

    for _ in 0..size - 1 {
        if start == input {
            return (x.abs() + y.abs()) as u32;
        }
        start += 1;
        x -= 1;
    }

    for _ in 0..size - 1 {
        if start == input {
            return (x.abs() + y.abs()) as u32;
        }
        start += 1;
        y -= 1;
    }

    for _ in 0..size - 1 {
        if start == input {
            return (x.abs() + y.abs()) as u32;
        }
        start += 1;
        x += 1;
    }

    return (x.abs() + y.abs()) as u32;
}

fn part2(input: u32) -> u32 {
    let mut numbers = vec![1, 1, 2, 4, 5, 10, 11, 23, 25];

    let mut high_index = 8;
    let mut low_index = 1;
    let mut ring_size = 5;
    loop {
        {
            let found = numbers.iter().find(|&&n| n > input);
            if found.is_some() {
                return *found.unwrap();
            }
        }
        // Do this for each side
        for i in 0..4 {
            // 1st corner
            let n = numbers[high_index] + numbers[low_index];
            numbers.push(n);
            high_index += 1;

            // 1st after corner
            let n = numbers[high_index] + numbers[high_index - 1] + numbers[low_index] +
                numbers[low_index + 1];
            numbers.push(n);
            high_index += 1;


            // Side stretch
            // Minus 4 because corners and neighbors of corners are special cases already handled in outer loop
            let mut side_count = ring_size - 4;
            // First side is a special case b/c of the spiral. So we have one less on the side
            if i == 0 {
                side_count -= 1
            }
            // Last side is a special case b/c of the spiral. So we have one extra on the side
            if i == 3 {
                side_count += 1
            }
            for _ in 0..side_count {
                let n = numbers[high_index] + numbers[low_index] + numbers[low_index + 1] +
                    numbers[low_index + 2];
                numbers.push(n);
                high_index += 1;
                low_index += 1;
            }

            // Before final corner
            let n = numbers[high_index] + numbers[low_index] + numbers[low_index + 1];
            numbers.push(n);
            high_index += 1;
            low_index += 1;
        }
        ring_size += 2;
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(0, part1(1));
    }

    #[test]
    fn example12() {
        assert_eq!(3, part1(12));
    }

    #[test]
    fn example23() {
        assert_eq!(2, part1(23));
    }

    #[test]
    fn example1024() {
        assert_eq!(31, part1(1024));
    }

    #[test]
    fn solution() {
        assert_eq!(371, part1(INPUT));
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(59, part2(58));
    }

    #[test]
    fn example2() {
        assert_eq!(122, part2(59));
    }

    #[test]
    fn example3() {
        assert_eq!(806, part2(800));
    }

    #[test]
    fn example4() {
        assert_eq!(880, part2(807));
    }

    #[test]
    fn example5() {
        assert_eq!(931, part2(881));
    }

    #[test]
    fn solution() {
        assert_eq!(369601, part2(INPUT));
    }
}

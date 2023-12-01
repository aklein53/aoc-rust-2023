advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let first = first_non_digit(line.chars().collect::<String>());
        let last = first_non_digit(line.chars().rev().collect::<String>());

        [first, last].iter().collect::<String>().parse::<u32>().unwrap()
    }).sum())
}

fn first_non_digit(input: String) -> char {
    let test = String::from("123456789");
    let test2 = &test[0..1];
    let len = input.len();
    let mut cur = 0;
    while cur < len {
        if &input[cur..cur+1] == "1" {return '1';}
        if &input[cur..cur+1] == "2" {return '2';}
        if &input[cur..cur+1] == "3" {return '3';}
        if &input[cur..cur+1] == "4" {return '4';}
        if &input[cur..cur+1] == "5" {return '5';}
        if &input[cur..cur+1] == "6" {return '6';}
        if &input[cur..cur+1] == "7" {return '7';}
        if &input[cur..cur+1] == "8" {return '8';}
        if &input[cur..cur+1] == "9" {return '9';}
        cur += 1;
    }
    return '0';
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let first = first_non_digit_pt2(line.chars().collect::<String>(), false);
        let last = first_non_digit_pt2(line.chars().rev().collect::<String>(), true);

        [first, last].iter().collect::<String>().parse::<u32>().unwrap()
    }).sum())
}

// Find the first instance of any of the strings in a dictionary, and return the associated value
fn 

fn first_non_digit_pt2(input: String, reverse: bool) -> char {
    let len = input.len();
    let mut cur = 0;
    while cur < len {
        if !reverse {
            if &input[cur..cur+1] == "1" {return '1';}
            if &input[cur..cur+1] == "2" {return '2';}
            if &input[cur..cur+1] == "3" {return '3';}
            if &input[cur..cur+1] == "4" {return '4';}
            if &input[cur..cur+1] == "5" {return '5';}
            if &input[cur..cur+1] == "6" {return '6';}
            if &input[cur..cur+1] == "7" {return '7';}
            if &input[cur..cur+1] == "8" {return '8';}
            if &input[cur..cur+1] == "9" {return '9';}
            if cur+3 < len && &input[cur..cur+3] == "one" {return '1';}
            if cur+3 < len && &input[cur..cur+3] == "two" {return '2';}
            if cur+5 < len && &input[cur..cur+5] == "three" {return '3';}
            if cur+4 < len && &input[cur..cur+4] == "four" {return '4';}
            if cur+4 < len && &input[cur..cur+4] == "five" {return '5';}
            if cur+3 < len && &input[cur..cur+3] == "six" {return '6';}
            if cur+5 < len && &input[cur..cur+5] == "seven" {return '7';}
            if cur+5 < len && &input[cur..cur+5] == "eight" {return '8';}
            if cur+4 < len && &input[cur..cur+4] == "nine" {return '9';}
        } else {
            if &input[cur..cur+1] == "1" {return '1';}
            if &input[cur..cur+1] == "2" {return '2';}
            if &input[cur..cur+1] == "3" {return '3';}
            if &input[cur..cur+1] == "4" {return '4';}
            if &input[cur..cur+1] == "5" {return '5';}
            if &input[cur..cur+1] == "6" {return '6';}
            if &input[cur..cur+1] == "7" {return '7';}
            if &input[cur..cur+1] == "8" {return '8';}
            if &input[cur..cur+1] == "9" {return '9';}
            if cur+3 < len && &input[cur..cur+3] == "eno" {return '1';}
            if cur+3 < len && &input[cur..cur+3] == "owt" {return '2';}
            if cur+5 < len && &input[cur..cur+5] == "eerht" {return '3';}
            if cur+4 < len && &input[cur..cur+4] == "ruof" {return '4';}
            if cur+4 < len && &input[cur..cur+4] == "evif" {return '5';}
            if cur+3 < len && &input[cur..cur+3] == "xis" {return '6';}
            if cur+5 < len && &input[cur..cur+5] == "neves" {return '7';}
            if cur+5 < len && &input[cur..cur+5] == "thgie" {return '8';}
            if cur+4 < len && &input[cur..cur+4] == "enin" {return '9';}
        }
        cur += 1;
    }
    return '0';
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

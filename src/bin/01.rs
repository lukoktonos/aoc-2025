advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut times_zero_seen = 0;
    let mut dial_pos: i64 = 50;
    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        if line.strip_prefix('R').is_some() {
            dial_pos += line[1..].parse::<i64>().unwrap();
        }

        if line.strip_prefix('L').is_some() {
            dial_pos -= line[1..].parse::<i64>().unwrap();
        }
        while dial_pos < 0 {
            dial_pos += 100;
        }
        while dial_pos >= 100 {
            dial_pos -= 100;
        }
        if dial_pos == 0 {
            times_zero_seen += 1;
        }
    }

    Some(times_zero_seen)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut times_zero_seen = 0;
    let mut dial_pos: i64 = 50;
    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        if line.strip_prefix('R').is_some() {
            perform_clicks(
                line[1..].parse::<i64>().unwrap(),
                &mut dial_pos,
                &mut times_zero_seen,
            );
        }

        if line.strip_prefix('L').is_some() {
            perform_clicks(
                -line[1..].parse::<i64>().unwrap(),
                &mut dial_pos,
                &mut times_zero_seen,
            );
        }
    }

    Some(times_zero_seen)
}

fn perform_clicks(num_clicks: i64, dial_pos: &mut i64, zeros: &mut u64) {
    if num_clicks > 0 {
        for _ in 0..num_clicks {
            *dial_pos += 1;
            if *dial_pos == 100 {
                *dial_pos = 0;
            };
            if *dial_pos == 0 {
                *zeros += 1;
            }
        }
    } else {
        for _ in num_clicks..0 {
            *dial_pos -= 1;
            if *dial_pos == -1 {
                *dial_pos = 99;
            }
            if *dial_pos == 0 {
                *zeros += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6));
    }
}

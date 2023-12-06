/// I originally solved this day's challenges by hand, but have also written the solution in code,
/// for posterity.
///
/// # Solution explain:
///
/// The formula for the distance a boat will travel is simple:
/// d = s * (T - h)
/// where d is the distance travelled, s is the speed of the boat, T is the total race time, and h
/// is the time that the button was held for. Since the speed of the boat is identical to the time
/// that the button was held for, s can be replaced with h and the formula can be rearranged to the
/// following:
/// h^2 - T * h + d = 0
/// We know the minimum distance the boat must travel, and the total race time, so the only unknown
/// is h. Clearly, this is a quadratic function, and, thus, can be solved via the quadratic
/// formula.
/// h = (T ± sqrt(T^2 - 4 * d) / 2)
/// This will give us the minimum and maximum hold times required to win the race. From there, all
/// we need to do is subtract the minimum from the maximum and we have calculated the total number
/// of ways of winning the race.

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> u64 {
    parse_input_1(input)
        .into_iter()
        .map(number_of_ways_to_win)
        .product()
}

fn run_2(input: &str) -> u64 {
    number_of_ways_to_win(parse_input_2(input))
}

fn parse_input_1(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once('\n').unwrap();
    parse_line_1(times)
        .zip(parse_line_1(distances))
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_line_1(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
}

fn parse_input_2(input: &str) -> Race {
    let (times, distances) = input.split_once('\n').unwrap();
    Race {
        time: parse_line_2(times),
        distance: parse_line_2(distances),
    }
}

fn parse_line_2(line: &str) -> u64 {
    line.split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap()
}

fn number_of_ways_to_win(race: Race) -> u64 {
    let min = (race.time as f64 - ((race.time.pow(2) - 4 * race.distance) as f64).sqrt()) / 2.0;
    let max = (race.time as f64 + ((race.time.pow(2) - 4 * race.distance) as f64).sqrt()) / 2.0;
    (max - 1.0).ceil() as u64 - min as u64
}

struct Race {
    time: u64,
    distance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 288);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 71503);
    }
}

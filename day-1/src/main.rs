fn main() {
    const INPUT: &[u8; 21580] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &[u8]) -> u64 {
    run(input, first_digit_1, last_digit_1)
}

fn run_2(input: &[u8]) -> u64 {
    run(input, first_digit_2, last_digit_2)
}

#[inline]
fn run(input: &[u8], first_digit: fn(&[u8]) -> u64, last_digit: fn(&[u8]) -> u64) -> u64 {
    input
        .split(|c| *c == b'\n')
        .map(|line| read_line(line, first_digit, last_digit))
        .sum()
}

fn read_line(line: &[u8], first_digit: fn(&[u8]) -> u64, last_digit: fn(&[u8]) -> u64) -> u64 {
    first_digit(line) * 10 + last_digit(line)
}

fn first_digit_1(chars: &[u8]) -> u64 {
    digit_1(chars)
}

fn last_digit_1(chars: &[u8]) -> u64 {
    digit_1(chars.iter().rev())
}

fn digit_1<'a>(chars: impl IntoIterator<Item = &'a u8>) -> u64 {
    for c in chars {
        if *c >= b'0' && *c <= b'9' {
            return to_digit(*c);
        }
    }
    panic!("No digit found")
}

fn to_digit(c: u8) -> u64 {
    (c - b'0') as u64
}

fn first_digit_2(chars: &[u8]) -> u64 {
    for i in 0..chars.len() {
        if let Some(digit) = get_digit(chars[i..].iter()) {
            return digit;
        }
    }
    panic!("No digit found")
}

fn last_digit_2(chars: &[u8]) -> u64 {
    for i in (0..chars.len()).rev() {
        if let Some(digit) = get_digit(chars[i..].iter()) {
            return digit;
        }
    }
    panic!("No digit found")
}

fn get_digit<'a>(mut chars: impl Iterator<Item = &'a u8>) -> Option<u64> {
    match chars.next() {
        Some(c @ b'0'..=b'9') => Some((*c - b'0') as u64),
        Some(b'o') => expect(chars, b"ne", 1),
        Some(b't') => match chars.next() {
            Some(b'w') => expect(chars, b"o", 2),
            Some(b'h') => expect(chars, b"ree", 3),
            _ => None,
        },
        Some(b'f') => match chars.next() {
            Some(b'o') => expect(chars, b"ur", 4),
            Some(b'i') => expect(chars, b"ve", 5),
            _ => None,
        },
        Some(b's') => match chars.next() {
            Some(b'i') => expect(chars, b"x", 6),
            Some(b'e') => expect(chars, b"ven", 7),
            _ => None,
        },
        Some(b'e') => expect(chars, b"ight", 8),
        Some(b'n') => expect(chars, b"ine", 9),
        _ => None,
    }
}

fn expect<'a>(mut chars: impl Iterator<Item = &'a u8>, expected: &[u8], digit: u64) -> Option<u64> {
    for e in expected {
        if chars.next() != Some(e) {
            return None;
        }
    }
    Some(digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT: &[u8; 40] = b"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(run_1(INPUT), 142);
    }

    #[test]
    fn challenge_2() {
        const INPUT: &[u8; 92] = b"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(run_2(INPUT), 281);
    }
}

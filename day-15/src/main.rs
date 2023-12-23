fn main() {
    const INPUT: &[u8; 22816] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    input.split(|b| *b == b',').map(hash).sum()
}

fn run_2(input: &[u8]) -> usize {
    let mut hashmap = vec![Vec::<(&[u8], u8)>::new(); 256];
    for step in input.split(|b| *b == b',') {
        let (last, rest) = step.split_last().unwrap();
        match last {
            focal_length @ b'1'..=b'9' => {
                // Must be a '=' operation.
                let label = &rest[..rest.len() - 1];
                let hash = hash(label);
                let lenses = hashmap.get_mut(hash).unwrap();
                if let Some(lens) = lenses.iter_mut().find(|(other, _)| *other == label) {
                    lens.1 = *focal_length;
                } else {
                    lenses.push((label, *focal_length));
                }
            }
            _ => {
                // Must be a '-' operation.
                let label = rest;
                let hash = hash(label);
                let lenses = hashmap.get_mut(hash).unwrap();
                if let Some(position) = lenses.iter().position(|(other, _)| *other == label) {
                    lenses.remove(position);
                }
            }
        }
    }
    hashmap
        .into_iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| {
                    (1 + box_idx) * (lens_idx + 1) * (focal_length - b'0') as usize
                })
                .sum::<usize>()
        })
        .sum()
}

fn hash(step: &[u8]) -> usize {
    let mut hash: u8 = 0;
    for byte in step {
        hash = hash.wrapping_add(*byte);
        hash = hash.wrapping_mul(17);
    }
    hash as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8; 51] = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 1320);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 145);
    }

    #[test]
    fn h() {
        assert_eq!(hash(b"HASH"), 52);
    }
}

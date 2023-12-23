fn main() {
    const INPUT: &[u8; 22816] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    input.split(|b| *b == b',').map(hash).sum()
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
    fn h() {
        assert_eq!(hash(b"HASH"), 52);
    }
}

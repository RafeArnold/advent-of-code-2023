use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

fn main() {
    const INPUT: &[u8; 20021] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let grid = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    let start = (0, 0);
    let finish = (grid.len() - 1, grid[0].len() - 1);
    let mut start_state = State::new(start, finish);
    start_state.set_distance_from(finish);

    let mut queue = BinaryHeap::new();
    queue.push(Reverse(start_state));

    let mut visited = HashSet::<(usize, usize)>::new();

    loop {
        let state = queue.pop().unwrap().0;

        // draw(&state, &grid);
        if state.pos == finish {
            return state.heat_loss;
        }

        visited.insert(state.pos);

        let moves = HashSet::<(isize, isize)>::from([(0, -1), (-1, 0), (0, 1), (1, 0)]);
        let diff = if let Some(last) = state.path.last() {
            let (dx, dy) = (
                state.pos.0 as isize - last.0 as isize,
                state.pos.1 as isize - last.1 as isize,
            );
            HashSet::from([(dx, dy), (-dx, -dy)])
        } else {
            HashSet::new()
        };

        for &(dx, dy) in moves.difference(&diff) {
            let mut prev_state = state.clone();

            for _ in 0..3 {
                if let Some(new_state) = prev_state.try_move((dx, dy), &grid, finish) {
                    if !visited.contains(&new_state.pos) {
                        queue.push(Reverse(new_state.clone()));

                        prev_state = new_state.clone();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn draw(state: &State, grid: &[&[u8]]) {
    if state.path.is_empty() {
        return;
    }
    let mut new_grid = Vec::new();
    for row in grid {
        let mut new_row = Vec::new();
        for tile in *row {
            new_row.push(*tile as char);
        }
        new_grid.push(new_row);
    }
    for window in state.path.windows(2) {
        put_char(window[0], window[1], &mut new_grid);
    }
    put_char(*state.path.last().unwrap(), state.pos, &mut new_grid);
    new_grid[state.pos.1][state.pos.0] = '.';
    let str = new_grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    println!(
        "\n{}",
        &state
            .path
            .iter()
            .map(|(dx, dy)| format!("({dx}, {dy})"))
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!("{str}");
}

fn put_char(old_pos: (usize, usize), new_pos: (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (dx, dy) = (
        new_pos.0 as isize - old_pos.0 as isize,
        new_pos.1 as isize - old_pos.1 as isize,
    );
    let char = match (dx, dy) {
        (0, -1) => '^',
        (0, 1) => 'v',
        (-1, 0) => '<',
        (1, 0) => '>',
        _ => '.',
    };
    grid[old_pos.1][old_pos.0] = char;
}

#[derive(Clone)]
struct State {
    heat_loss: usize,
    distance_from_finish: usize,
    pos: (usize, usize),
    path: Vec<(usize, usize)>,
}

impl State {
    fn new(pos: (usize, usize), finish: (usize, usize)) -> Self {
        let mut s = Self {
            heat_loss: 0,
            distance_from_finish: 0,
            pos,
            path: vec![],
        };
        s.set_distance_from(finish);
        s
    }

    fn try_move(
        &self,
        (dx, dy): (isize, isize),
        grid: &[&[u8]],
        finish: (usize, usize),
    ) -> Option<State> {
        let mut new_state = self.clone();
        new_state.pos.0 = try_move_single(new_state.pos.0, dx, grid[0].len())?;
        new_state.pos.1 = try_move_single(new_state.pos.1, dy, grid.len())?;
        new_state.path.push(self.pos);
        new_state.add_heat_loss(&grid);
        new_state.set_distance_from(finish);
        Some(new_state)
    }

    fn add_heat_loss(&mut self, grid: &[&[u8]]) {
        self.heat_loss += (grid[self.pos.1][self.pos.0] - b'0') as usize;
    }

    fn set_distance_from(&mut self, from: (usize, usize)) {
        self.distance_from_finish = self.pos.0.abs_diff(from.0) + self.pos.1.abs_diff(from.1);
    }
}

fn try_move_single(p: usize, d: isize, max: usize) -> Option<usize> {
    if let Some(new_p) = p.checked_add_signed(d) {
        if new_p < max {
            Some(new_p)
        } else {
            None
        }
    } else {
        None
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.heat_loss + self.distance_from_finish)
            .cmp(&(other.heat_loss + other.distance_from_finish))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss + self.distance_from_finish == other.heat_loss + other.distance_from_finish
            && self.pos == other.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8; 181] = br#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 102);
    }

    #[test]
    fn cmp() {
        let a = State {
            heat_loss: 15,
            distance_from_finish: 20,
            pos: (3, 1),
            path: vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 0), (3, 0), (3, 1)],
        };
        let b = State {
            heat_loss: 11,
            distance_from_finish: 20,
            pos: (3, 1),
            path: vec![(0, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
        };
        assert_eq!(b.cmp(&a), Ordering::Less);
    }
}

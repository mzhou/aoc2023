use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::io::stdin;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    direction: Direction,
    direction_count: u8,
    pos: usize,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct StateWithCost {
    cost: usize,
    state: State,
}

impl Ord for StateWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for StateWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Vec::<u8>::new();
    let mut width = 0;

    for line in stdin().lines() {
        let line = line?;
        let line_bytes = line.as_bytes();

        if width == 0 {
            width = line_bytes.len();
        } else {
            if width != line_bytes.len() {
                panic!("width mismatch");
            }
        }

        grid.extend(line_bytes.iter().map(|b| b - b'0'));
    }

    let count = shortest_path(
        &grid,
        width,
        &State {
            direction: Direction::Right,
            direction_count: 0,
            pos: 0,
        },
    );

    println!("{}", count);

    Ok(())
}

fn shortest_path(grid: &[u8], width: usize, starting_state: &State) -> usize {
    let mut candidates = BinaryHeap::<Reverse<StateWithCost>>::new();
    let mut visited = HashSet::<State>::new();

    candidates.push(Reverse(StateWithCost {
        cost: 0,
        state: starting_state.clone(),
    }));

    while let Some(Reverse(candidate)) = candidates.pop() {
        if !visited.insert(candidate.state.clone()) {
            continue;
        }

        if candidate.state.pos == grid.len() - 1 && candidate.state.direction_count >= 4 {
            return candidate.cost;
        }

        let new_direction_count = |new_direction: Direction| {
            if new_direction == candidate.state.direction {
                candidate.state.direction_count + 1
            } else {
                1
            }
        };

        if candidate.state.pos < grid.len() - width
            && candidate.state.direction != Direction::Up
            && ((candidate.state.direction == Direction::Down
                && candidate.state.direction_count < 10)
                || (candidate.state.direction != Direction::Down
                    && candidate.state.direction_count >= 4))
        {
            let new_pos = candidate.state.pos + width;
            candidates.push(Reverse(StateWithCost {
                cost: candidate.cost + grid[new_pos] as usize,
                state: State {
                    direction: Direction::Down,
                    direction_count: new_direction_count(Direction::Down),
                    pos: new_pos,
                },
            }));
        }

        if candidate.state.pos % width != 0
            && candidate.state.direction != Direction::Right
            && ((candidate.state.direction == Direction::Left
                && candidate.state.direction_count < 10)
                || (candidate.state.direction != Direction::Left
                    && candidate.state.direction_count >= 4))
        {
            let new_pos = candidate.state.pos - 1;
            candidates.push(Reverse(StateWithCost {
                cost: candidate.cost + grid[new_pos] as usize,
                state: State {
                    direction: Direction::Left,
                    direction_count: new_direction_count(Direction::Left),
                    pos: new_pos,
                },
            }));
        }

        if candidate.state.pos % width != width - 1
            && candidate.state.direction != Direction::Left
            && ((candidate.state.direction == Direction::Right
                && candidate.state.direction_count < 10)
                || (candidate.state.direction != Direction::Right
                    && candidate.state.direction_count >= 4))
        {
            let new_pos = candidate.state.pos + 1;
            candidates.push(Reverse(StateWithCost {
                cost: candidate.cost + grid[new_pos] as usize,
                state: State {
                    direction: Direction::Right,
                    direction_count: new_direction_count(Direction::Right),
                    pos: new_pos,
                },
            }));
        }

        if candidate.state.pos >= width
            && candidate.state.direction != Direction::Down
            && ((candidate.state.direction == Direction::Up
                && candidate.state.direction_count < 10)
                || (candidate.state.direction != Direction::Up
                    && candidate.state.direction_count >= 4))
        {
            let new_pos = candidate.state.pos - width;
            candidates.push(Reverse(StateWithCost {
                cost: candidate.cost + grid[new_pos] as usize,
                state: State {
                    direction: Direction::Up,
                    direction_count: new_direction_count(Direction::Up),
                    pos: new_pos,
                },
            }));
        }
    }

    panic!("no path found");
}

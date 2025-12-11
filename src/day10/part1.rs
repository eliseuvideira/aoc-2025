use std::collections::{HashSet, VecDeque};

use anyhow::{Context, Result};

use super::{Button, Lights, Machine, parse_machines};

fn apply_button(state: &[Lights], button: &Button) -> Vec<Lights> {
    let mut new_state = state.to_vec();

    for index in button {
        new_state[*index as usize] = match state[*index as usize] {
            Lights::On => Lights::Off,
            Lights::Off => Lights::On,
        };
    }

    new_state
}

fn min_total_presses(machine: &Machine) -> Option<u64> {
    let initial_state = vec![Lights::Off; machine.indicator.len()];

    let mut queue = VecDeque::from([(initial_state.clone(), 0)]);
    let mut visited = HashSet::from([initial_state]);

    while let Some((state, presses)) = queue.pop_front() {
        if state == machine.indicator {
            return Some(presses);
        }

        for button in &machine.buttons {
            let new_state = apply_button(&state, button);
            if visited.insert(new_state.clone()) {
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    None
}

pub fn run(input: &str) -> Result<String> {
    let machines = parse_machines(input)?;

    let min_total_presses = machines
        .iter()
        .map(|machine| min_total_presses(machine).context("no solution"))
        .sum::<Result<u64>>()?;

    Ok(min_total_presses.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(run(input)?, "7".to_string());

        Ok(())
    }
}

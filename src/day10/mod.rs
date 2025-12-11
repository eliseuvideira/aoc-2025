use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;


#[derive(Debug)]
pub struct Machine {
    pub indicator: Vec<Lights>,
    pub buttons: Vec<Button>,
    pub joltage_requirements: Vec<Joltage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lights {
    On,
    Off,
}

pub type Button = Vec<u8>;

pub type Joltage = u16;

fn parse_indicator(indicator: &str) -> Vec<Lights> {
    indicator
        .chars()
        .filter_map(|c| match c {
            '.' => Some(Lights::Off),
            '#' => Some(Lights::On),
            _ => None,
        })
        .collect::<Vec<_>>()
}

fn parse_button(button: &str) -> Result<Button> {
    button[1..button.len() - 1]
        .split(',')
        .map(|s| s.parse::<u8>().context("expected number"))
        .collect::<Result<Vec<_>>>()
}

fn parse_buttons(buttons: Vec<&str>) -> Result<Vec<Button>> {
    buttons
        .into_iter()
        .map(parse_button)
        .collect::<Result<Vec<_>>>()
}

fn parse_joltage_requirements(joltage_requirements: &str) -> Result<Vec<Joltage>> {
    joltage_requirements
        .split(',')
        .map(|s| s.parse::<Joltage>().context("expected number"))
        .collect::<Result<Vec<_>>>()
}

pub fn parse_machine(line: &str) -> Result<Machine> {
    let (indicator, rhs) = line.split_once(' ').context("expected empty space")?;
    let joltage_index = rhs.find('{').context("expected opening brace")?;

    let indicator = parse_indicator(indicator);

    let buttons = rhs[0..joltage_index - 1].split(' ').collect::<Vec<_>>();
    let buttons = parse_buttons(buttons)?;

    let joltage_requirements = parse_joltage_requirements(&rhs[joltage_index + 1..rhs.len() - 1])?;

    Ok(Machine {
        indicator,
        buttons,
        joltage_requirements,
    })
}

pub fn parse_machines(input: &str) -> Result<Vec<Machine>> {
    input.lines().map(parse_machine).collect::<Result<Vec<_>>>()
}

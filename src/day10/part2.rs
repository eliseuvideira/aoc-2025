use anyhow::Result;
use std::fmt;
use std::ops::{Div, Mul, Sub};

#[cfg(debug_assertions)]
use super::{Button, Joltage};
use super::{Machine, parse_machines};

#[derive(Clone, Copy)]
struct Fraction(i64, i64);

impl Default for Fraction {
    fn default() -> Self {
        Self(0, 1)
    }
}

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 % self.1 == 0 {
            write!(f, "{}", self.0 / self.1)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

impl From<i64> for Fraction {
    fn from(value: i64) -> Self {
        Self(value, 1)
    }
}

impl From<Fraction> for i64 {
    fn from(value: Fraction) -> Self {
        value.0 / value.1
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

impl Fraction {
    #[must_use]
    fn simplify(self) -> Self {
        if self.0 == 0 {
            return Self(0, 1);
        }
        let g = gcd(self.0, self.1);
        Self(self.0 / g, self.1 / g)
    }

    #[must_use]
    fn is_zero(self) -> bool {
        self.0 == 0 && self.1 != 0
    }

    #[must_use]
    fn is_integer(self) -> bool {
        self.1 != 0 && self.0 % self.1 == 0
    }
}

impl Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1).simplify()
    }
}

impl Sub for Fraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1).simplify()
    }
}

impl Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self(self.0 * rhs.1, self.1 * rhs.0).simplify()
    }
}

struct Equation {
    elements: Vec<Fraction>,
    constant: Fraction,
}

impl fmt::Debug for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {}",
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" + "),
            self.constant
        )?;

        Ok(())
    }
}

impl Equation {
    fn mul_const(&self, value: Fraction) -> Self {
        Self {
            elements: self.elements.iter().map(|&e| e * value).collect(),
            constant: self.constant * value,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.elements.len(), other.elements.len());
        Self {
            elements: self
                .elements
                .iter()
                .zip(other.elements.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
            constant: self.constant - other.constant,
        }
    }
}

fn find_leading_col(equation: &Equation) -> Option<usize> {
    equation.elements.iter().position(|&e| !e.is_zero())
}

struct Formula {
    variable: usize,
    constant: Fraction,
    terms: Vec<(usize, Fraction)>,
}

impl Formula {
    fn from_equation(equation: &Equation, leading_col: usize, unbound_cols: &[usize]) -> Self {
        let divisor = equation.elements[leading_col];
        Self {
            variable: leading_col,
            constant: equation.constant / divisor,
            terms: unbound_cols
                .iter()
                .map(|&col| (col, equation.elements[col] / divisor))
                .collect(),
        }
    }

    fn evaluate(&self, unbound_values: &[i64]) -> Option<i64> {
        let mut result = self.constant;

        for (&(_, coeff), &val) in self.terms.iter().zip(unbound_values.iter()) {
            let term = coeff * Fraction::from(val);
            result = result - term;
        }

        if !result.is_integer() {
            return None;
        }
        Some(result.into())
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let constant: i64 = self.constant.into();
        write!(f, "x{} = {}", self.variable, constant)?;
        for &(col, coeff) in &self.terms {
            if coeff.is_zero() {
                continue;
            }
            let val: i64 = coeff.into();
            if val == 1 {
                write!(f, " - x{}", col)?;
            } else if val == -1 {
                write!(f, " + x{}", col)?;
            } else if val > 0 {
                write!(f, " - {}*x{}", val, col)?;
            } else {
                write!(f, " + {}*x{}", -val, col)?;
            }
        }
        Ok(())
    }
}

#[cfg(debug_assertions)]
fn print_formulas(formulas: &[Formula]) {
    for formula in formulas {
        println!("{}", formula);
    }
    println!();
}

#[cfg(debug_assertions)]
fn print_bound_unbound(bound_cols: &[usize], unbound_cols: &[usize], num_cols: usize) {
    print!(" B => ");
    for i in 0..num_cols {
        if bound_cols.contains(&i) {
            print!("{:>4} ", format!("x{}", i));
        } else {
            print!("     ");
        }
    }
    println!();

    print!(" U => ");
    for i in 0..num_cols {
        if unbound_cols.contains(&i) {
            print!("{:>4} ", format!("x{}", i));
        } else {
            print!("     ");
        }
    }
    println!();
    println!();
}

#[cfg(debug_assertions)]
fn print_solutions_header(num_cols: usize) {
    print!("      ");
    for i in 0..num_cols {
        print!("{:>4} ", format!("x{}", i));
    }
    println!("             S");
}

#[cfg(debug_assertions)]
fn print_minimum(num_cols: usize, min: i64) {
    print!("      ");
    for _ in 0..num_cols {
        print!("     ");
    }
    println!("min =>    {:4}", min);
    println!();
}

#[cfg(debug_assertions)]
fn print_minimum_presses(
    formulas: &[Formula],
    unbound_values: &[i64],
    solution: &[(usize, i64)],
    num_cols: usize,
    sum: i64,
) {
    let mut values = vec![0i64; num_cols];

    if let Some(first) = formulas.first() {
        for ((col, _), &val) in first.terms.iter().zip(unbound_values.iter()) {
            values[*col] = val;
        }
    }
    for (var, val) in solution {
        values[*var] = *val;
    }

    print!("      ");
    for val in &values {
        print!("{:4} ", val);
    }
    print!("    =>    {:4}", sum);

    if let Some(first) = formulas.first() {
        print!("   (");
        for (i, ((col, _), &val)) in first.terms.iter().zip(unbound_values.iter()).enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("x{}={}", col, val);
        }
        print!(")");
    }
    println!();
}

fn find_minimum_presses(
    formulas: &[Formula],
    unbound_values: &mut Vec<i64>,
    depth: usize,
    #[cfg(debug_assertions)] num_cols: usize,
    max_val: i64,
    current_best: Option<i64>,
) -> Option<i64> {
    let partial_sum: i64 = unbound_values.iter().sum();
    if let Some(best) = current_best
        && partial_sum >= best
    {
        return None;
    }

    if depth == formulas.first().map_or(0, |f| f.terms.len()) {
        let mut sum = partial_sum;
        #[cfg(debug_assertions)]
        let mut solution = Vec::new();

        for formula in formulas {
            let val = formula.evaluate(unbound_values)?;
            if val < 0 {
                return None;
            }
            if let Some(best) = current_best
                && sum + val >= best
            {
                return None;
            }
            #[cfg(debug_assertions)]
            solution.push((formula.variable, val));
            sum += val;
        }

        #[cfg(debug_assertions)]
        print_minimum_presses(formulas, unbound_values, &solution, num_cols, sum);

        return Some(sum);
    }

    let mut min = current_best;
    for v in 0..=max_val {
        unbound_values.push(v);
        if let Some(result) = find_minimum_presses(
            formulas,
            unbound_values,
            depth + 1,
            #[cfg(debug_assertions)]
            num_cols,
            max_val,
            min,
        ) {
            min = Some(min.map_or(result, |m| m.min(result)));
        }
        unbound_values.pop();
    }
    min
}

#[cfg(debug_assertions)]
fn print_buttons_and_joltages(buttons: &[Button], joltage_requirements: &[Joltage]) {
    print!("      ");
    for i in 0..joltage_requirements.len() {
        print!("{:>4} ", format!("c{}", i));
    }
    println!();

    for (i, button) in buttons.iter().enumerate() {
        print!("x{} => ", i);
        for j in 0..joltage_requirements.len() {
            if button.contains(&(j as u8)) {
                print!("{:4} ", 1);
            } else {
                print!("{:4} ", 0);
            }
        }
        println!();
    }

    print!("      ");
    for joltage in joltage_requirements {
        print!("{:4} ", joltage);
    }
    println!();
    println!();
}

#[cfg(debug_assertions)]
fn print_matrix(buttons: &[Button], equations: &[Equation]) {
    print!("      ");
    for i in 0..buttons.len() {
        print!("{:>4} ", format!("x{}", i));
    }
    print!("         {:>4}", "J");
    println!();

    for (i, eq) in equations.iter().enumerate() {
        print!("c{} => ", i);
        for element in eq.elements.iter() {
            print!("{:4} ", i64::from(*element));
        }
        println!("    =    {:4}", i64::from(eq.constant));
    }
    println!();
}

fn min_presses_for_machine(machine: &Machine) -> i64 {
    #[cfg(debug_assertions)]
    print_buttons_and_joltages(&machine.buttons, &machine.joltage_requirements);

    let num_elements = machine.buttons.len();
    let mut equations = machine
        .joltage_requirements
        .iter()
        .map(|joltage| Equation {
            elements: vec![Fraction::default(); num_elements],
            constant: Fraction::from(*joltage as i64),
        })
        .collect::<Vec<_>>();

    for (i, button) in machine.buttons.iter().enumerate() {
        for j in button {
            equations[*j as usize].elements[i] = Fraction::from(1);
        }
    }

    #[cfg(debug_assertions)]
    print_matrix(&machine.buttons, &equations);

    let total_equations = equations.len();
    let mut current_row = 0;
    for column in 0..num_elements {
        let non_zeroed =
            (current_row..total_equations).find(|&i| !equations[i].elements[column].is_zero());

        let Some(found_row) = non_zeroed else {
            continue;
        };

        equations.swap(current_row, found_row);

        let current_element = equations[current_row].elements[column];
        for row in 0..total_equations {
            if row == current_row {
                continue;
            }

            let multiplier = equations[row].elements[column] / current_element;
            let equation = equations[current_row].mul_const(multiplier);
            equations[row] = equations[row].sub(&equation);
        }

        current_row += 1;
        if current_row >= total_equations {
            break;
        }
    }

    #[cfg(debug_assertions)]
    print_matrix(&machine.buttons, &equations);

    let leading_cols: Vec<Option<usize>> = equations.iter().map(find_leading_col).collect();

    let bound_cols: Vec<usize> = leading_cols.iter().filter_map(|&c| c).collect();
    let unbound_cols: Vec<usize> = (0..num_elements)
        .filter(|c| !bound_cols.contains(c))
        .collect();

    #[cfg(debug_assertions)]
    print_bound_unbound(&bound_cols, &unbound_cols, num_elements);

    let formulas: Vec<Formula> = equations
        .iter()
        .zip(leading_cols.iter())
        .filter_map(|(eq, leading_col)| {
            leading_col.map(|col| Formula::from_equation(eq, col, &unbound_cols))
        })
        .collect();

    #[cfg(debug_assertions)]
    print_formulas(&formulas);

    let max_val = machine
        .joltage_requirements
        .iter()
        .max()
        .copied()
        .unwrap_or(0) as i64;

    #[cfg(debug_assertions)]
    print_solutions_header(num_elements);
    let min = find_minimum_presses(
        &formulas,
        &mut Vec::new(),
        0,
        #[cfg(debug_assertions)]
        num_elements,
        max_val,
        None,
    )
    .unwrap_or(0);
    #[cfg(debug_assertions)]
    print_minimum(num_elements, min);
    min
}

pub fn run(input: &str) -> Result<String> {
    let machines = parse_machines(input)?;

    let total: i64 = machines.iter().map(min_presses_for_machine).sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(run(input)?, "33".to_string());

        Ok(())
    }
}

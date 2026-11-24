mod errors;

pub use errors::{InvalidAddressError, InvalidInstructionError, ProgramError};

pub type Program = Vec<i64>;

pub fn parse_intcode(raw: &str) -> Program {
    raw.trim()
        .split(',')
        .map(|x| {
            x.trim()
                .parse()
                .unwrap_or_else(|_| panic!("expected int, got {x}"))
        })
        .collect()
}

pub fn run_program(mut program: Program) -> Result<Program, ProgramError> {
    let mut i = 0;

    while let Some(&opcode) = program.get(i) {
        match opcode {
            1 => {
                run_operator(&mut program, i, |a, b| a + b)?;
                i += 4;
            }
            2 => {
                run_operator(&mut program, i, |a, b| a * b)?;
                i += 4;
            }
            99 => {
                break;
            }
            x => {
                return Err(InvalidInstructionError::new(x, i).into());
            }
        }
    }

    Ok(program)
}

fn get_code_params(
    program: &Program,
    i: usize,
) -> Result<(usize, usize, usize), InvalidAddressError> {
    let param = |offset| {
        program
            .get(i + offset)
            .and_then(|&v| usize::try_from(v).ok())
            .ok_or(InvalidAddressError::new(i + offset))
    };

    let (x, y, z) = (param(1)?, param(2)?, param(3)?);

    if let Some(bad) = [x, y, z].into_iter().find(|&a| a >= program.len()) {
        return Err(InvalidAddressError::new(bad));
    }

    Ok((x, y, z))
}

fn run_operator<F>(program: &mut Program, i: usize, operation: F) -> Result<(), ProgramError>
where
    F: Fn(i64, i64) -> i64,
{
    let (x, y, z) = get_code_params(program, i)?;
    let a = program[x];
    let b = program[y];

    program[z] = operation(a, b);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        for (input, expected) in [
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ] {
            assert_eq!(run_program(input).unwrap(), expected);
        }
    }
}

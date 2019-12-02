use std::fs;
use std::io;

fn load_program(filename: &str) -> io::Result<Vec<i32>> {
    Ok(
        fs::read_to_string(filename)?.split(",")
            .map(|v| v.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    )
}

fn run_program(program: &mut Vec<i32>) -> io::Result<()> {
    for i in (0..program.len()).step_by(4) {
        let result = match program[i] {
            1 => program[program[i+1] as usize] + program[program[i+2] as usize],
            2 => program[program[i+1] as usize] * program[program[i+2] as usize],
            99 => break,
            opcode => panic!("Invalid opcode: {}", opcode),
        };
        let output_index = program[i+3] as usize;
        program[output_index] = result;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let program = load_program("day2input.txt")?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            run_program(&mut program)?;

            if program[0] == 19690720 {
                println!("noun = {}, verb = {}", noun, verb);
                println!("answer = {}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        assert_eq!(run_program(&mut program).unwrap(), ());
        assert_eq!(program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        program = vec![1,0,0,0,99];
        assert_eq!(run_program(&mut program).unwrap(), ());
        assert_eq!(program, vec![2,0,0,0,99]);

        program = vec![2,3,0,3,99];
        assert_eq!(run_program(&mut program).unwrap(), ());
        assert_eq!(program, vec![2,3,0,6,99]);

        program = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(run_program(&mut program).unwrap(), ());
        assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
    }
}

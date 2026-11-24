const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));
const TARGET: i64 = 19690720;

fn main() {
    let program = intcode::parse_intcode(INPUT);

    // part 1

    let mut program_1 = program.clone();

    program_1[1] = 12;
    program_1[2] = 2;

    let output_1 = intcode::run_program(program_1).expect("part 1 program failed");

    let val = output_1[0];

    println!("output is {val}");

    // part 2

    for x in 0..100 {
        for y in 0..100 {
            let mut p = program.clone();
            p[1] = x;
            p[2] = y;

            let output = intcode::run_program(p);

            match output {
                Ok(out) => {
                    let val = out[0];

                    if val == TARGET {
                        let ans = x * 100 + y;
                        println!("answer is {ans}");
                        return;
                    }
                }
                Err(_) => continue,
            }
        }
    }

    println!("could not find a valid answer")
}

use adventofcode_2020::common::day_input_filename;
use adventofcode_2020::common::get_file_lines;
use adventofcode_2020::days;

fn test_day(day: u8, correct_a: &str, correct_b: &str) -> Result<(), std::io::Error> {
    let solve = days::get_solver(day).unwrap();
    let input_lines = get_file_lines(&day_input_filename(day))?;
    let (solution_a, solution_b) = solve(&input_lines);
    assert_eq!(
        solution_a.as_str(),
        correct_a,
        "Incorrect solution for day {}a",
        day
    );
    assert_eq!(
        solution_b.as_str(),
        correct_b,
        "Incorrect solution for day {}b",
        day
    );

    Ok(())
}

macro_rules! test_day {
    ($name: ident, $day: literal, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            test_day($day, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, 1, "542619", "32858450");

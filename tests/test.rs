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
    ($name: ident, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            let day_name = stringify!($name);
            let day_num: u8 = day_name[3..].parse().unwrap();
            test_day(day_num, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, "542619", "32858450");
test_day!(day02, "564", "325");
test_day!(day03, "268", "3093068400");
test_day!(day04, "250", "158");
test_day!(day05, "874", "594");
test_day!(day06, "6259", "3178");
test_day!(day07, "169", "82372");
test_day!(day08, "1087", "780");
test_day!(day09, "1212510616", "171265123");
test_day!(day10, "3034", "259172170858496");
test_day!(day11, "2424", "2208");
test_day!(day12, "1645", "35292");
test_day!(day13, "2995", "1012171816131114");
test_day!(day14, "10452688630537", "2881082759597");
test_day!(day15, "1111", "48568");
test_day!(day16, "20975", "910339449193");
test_day!(day17, "448", "2400");
test_day!(day18, "654686398176", "8952864356993");
test_day!(day19, "113", "253");
test_day!(day20, "7901522557967", "2476");
test_day!(day21, "2614", "qhvz,kbcpn,fzsl,mjzrj,bmj,mksmf,gptv,kgkrhg");
test_day!(day22, "35299", "33266");
test_day!(day23, "26354798", "166298218695");
test_day!(day24, "420", "4206");
test_day!(day25, "6408263", "");

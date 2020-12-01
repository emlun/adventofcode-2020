#![feature(test)]
extern crate test;
use adventofcode_2020::common::day_input_filename;
use adventofcode_2020::common::get_file_lines;
use adventofcode_2020::common::Solution;
use adventofcode_2020::days;
use test::Bencher;

macro_rules! run_bench {
    ($name: ident, $day: literal) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let input_lines = get_file_lines(&day_input_filename($day)).unwrap();
            let solve = days::get_solver($day).unwrap();
            b.iter(|| solve(&input_lines));
        }
    };
}

run_bench!(day01, 1);

#[bench]
fn days_all(b: &mut Bencher) {
    let solvers_and_inputs: Vec<(fn(&[String]) -> Solution, Vec<String>)> = days::all_numbers()
        .into_iter()
        .map(|day| {
            (
                days::get_solver(day).unwrap(),
                get_file_lines(&day_input_filename(day)).unwrap(),
            )
        })
        .collect();

    b.iter(|| {
        solvers_and_inputs
            .iter()
            .map(|(solver, input)| solver(&input))
            .collect::<Vec<Solution>>()
    })
}

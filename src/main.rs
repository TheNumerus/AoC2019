use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //day 1
    advent_2019::day_1::solve()?;

    // day 2 part 1
    advent_2019::day_2::solve_params(advent_2019::day_2::INPUT_GRAVITY, 12, 2);
    // day 2 part 2
    // guessing
    advent_2019::day_2::solve_params(advent_2019::day_2::INPUT_GRAVITY, 42, 59);

    advent_2019::day_3::solve();
    Ok(())
}
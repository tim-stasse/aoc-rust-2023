use anyhow::{bail, Context, Result};
use aoc::{day_1, day_2, day_3, day_4};
use clap::Parser;
use log::{debug, error};

#[derive(Parser)]
struct Cli {
    /// The advent day
    #[arg(short, long)]
    day: u8,
    /// The advent day
    #[arg(short, long)]
    puzzle: u8,
    /// The path to the input file
    #[arg(short, long)]
    input: std::path::PathBuf,
    /// The verbosity of the log level
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
    .filter_level(cli.verbosity.log_level_filter())
    .init();
    
    debug!(
        "Parsed Cli args: day: `{}`, puzzle: `{}`, input: `{}`, verbosity: `{}`",
        cli.day,
        cli.puzzle,
        cli.input.display(),
        cli.verbosity.log_level_filter()
    );

    let input = std::fs::read_to_string(&cli.input)
        .with_context(|| {
            error!("Could not read file `{}`", &cli.input.display());
            format!("could not read file `{}`", &cli.input.display())
        })?;

    match cli.day {
        1 => match cli.puzzle {
            1 => println!("{}", day_1::puzzle_1::solve(input.lines())?),
            2 => println!("{}", day_1::puzzle_2::solve(input.lines())?),
            _ => bail!("Unknown puzzle `{}`", cli.puzzle)
        },
        2 => match cli.puzzle {
            1 => println!("{}", day_2::puzzle_1::solve(input.lines())?),
            2 => println!("{}", day_2::puzzle_2::solve(input.lines())?),
            _ => bail!("Unknown puzzle `{}`", cli.puzzle)
        },
        3 => match cli.puzzle {
            1 => println!("{}", day_3::puzzle_1::solve(input.lines())?),
            2 => println!("{}", day_3::puzzle_2::solve(input.lines())?),
            _ => bail!("Unknown puzzle `{}`", cli.puzzle)
        },
        4 => match cli.puzzle {
            1 => println!("{}", day_4::puzzle_1::solve(input.lines())?),
            _ => bail!("Unknown puzzle `{}`", cli.puzzle)
        },
        _ => bail!("Unknown day `{}`", cli.day)
    };

    Ok(())
}

mod solution;

use project_euler_data::rust_template;

use clap::Parser;
use color_eyre::Result;
use std::{fs, path::Path, time::Instant};

#[derive(Debug, Parser)]
enum Cli {
    /// Run the solution to a problem
    Solve {
        /// Run a specific solution (if unspecified, run all available)
        problem: Option<u32>,
        /// Time the execution of the solution
        #[arg(short, long)]
        time: bool,
    },
    /// Generate Solution template
    Create {
        problem: u32,
        /// Time the execution of the solution
        #[arg(short, long)]
        force: bool,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    match args {
        Cli::Solve { problem, time } => {
            let problems = if let Some(problem) = problem.as_ref() {
                std::slice::from_ref(problem)
            } else {
                solution::AVAILABLE_SOLUTIONS
            };

            for problem in problems.iter().copied() {
                let now = Instant::now();
                let result = solution::solve(problem)?;
                let elapsed = now.elapsed();

                if time {
                    println!("Took {elapsed:?} to solve");
                }

                println!("The solution to Problem {problem} is:\n{result}\n");
            }
        }
        Cli::Create { problem, force } => {
            let file = Path::new("src")
                .join("solution")
                .join(format!("sol_{problem}.rs"));

            if file.exists() {
                if force {
                    eprintln!("{file:?} exists, overwriting");
                } else {
                    panic!("{file:?} exists, use --force to overwrite");
                }
            }

            fs::write(file, rust_template(problem))?;
        }
    }

    Ok(())
}

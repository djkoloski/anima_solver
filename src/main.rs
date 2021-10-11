use std::{env, fmt, fs, io, path::Path, time::Instant};

mod direction;
mod puzzle;
mod solve;
mod vec2;

pub use direction::*;
pub use puzzle::*;
pub use solve::solve;
pub use vec2::*;

struct DisplayState<'a>(&'a State, &'a Data);

impl<'a> fmt::Display for DisplayState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.display(self.1, f)
    }
}

struct Settings {
    verbose: bool,
    quiet: bool,
}

impl Settings {
    fn new() -> Self {
        Self {
            verbose: false,
            quiet: false,
        }
    }
}

pub fn execute() {
    let mut settings = Settings::new();
    let mut paths = Vec::new();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-v" => settings.verbose = true,
            "-q" => settings.quiet = true,
            _ => paths.push(arg),
        }
    }

    if paths.is_empty() {
        println!("Usage: {} [-v -q] PATHS", env::args().next().unwrap());
        println!("  -v       Print states along with solutions");
        println!("  -q       Do not print solutions");
        println!("  PATHS    A list of paths to problem files");
    } else {
        for path in paths {
            if let Err(e) = solve_file(path.as_ref(), &settings) {
                eprintln!("Error while solving '{}':\n{:?}", path, e);
            }
        }
    }
}

#[derive(Debug)]
enum SolveError<T> {
    IoError(io::Error),
    ParseError(T),
}

impl<T> From<io::Error> for SolveError<T> {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

fn solve_file(path: &Path, settings: &Settings) -> Result<(), SolveError<ParseError>> {
    let now = Instant::now();
    let (initial_state, data) =
        State::parse(&fs::read_to_string(path)?).map_err(SolveError::ParseError)?;
    let parse_elapsed = now.elapsed();

    let now = Instant::now();
    let result = solve(initial_state.clone(), &data);
    let solve_elapsed = now.elapsed();

    println!("{}:", path.to_str().unwrap());
    println!(
        "Parse: {}.{:09}s",
        parse_elapsed.as_secs(),
        parse_elapsed.subsec_nanos()
    );
    println!(
        "Solve: {}.{:09}s",
        solve_elapsed.as_secs(),
        solve_elapsed.subsec_nanos()
    );

    if !settings.quiet {
        if let Some(solution) = result {
            println!("Found solution of length {}:", solution.len());

            if settings.verbose {
                let mut state = initial_state;
                for action in solution {
                    println!("{}", DisplayState(&state, &data));
                    println!("{}", action);
                    if let Transition::Indeterminate(s) = state
                        .transitions(&data)
                        .into_iter()
                        .find(|(a, _)| a == &action)
                        .unwrap()
                        .1
                    {
                        state = s;
                    }
                }
            } else {
                let mut actions = solution.iter();
                if let Some(action) = actions.next() {
                    print!("{}", action);
                }
                for action in actions {
                    print!(", {}", action);
                }
                println!();
            }
        } else {
            println!("No solution");
        }
    }

    Ok(())
}

fn main() {
    execute();
}

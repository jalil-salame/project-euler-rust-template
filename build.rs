use color_eyre::Result;
use std::{env, fs, path::Path};

use hex::FromHex;
use project_euler_data::find_problem;

fn main() -> Result<()> {
    color_eyre::install()?;

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let sol_dir = Path::new(&manifest_dir).join("src").join("solution");
    let mut problems = vec![];
    for solution in sol_dir.read_dir()?.into_iter().filter_map(Result::ok) {
        let Some(problem) = parse_number(solution.file_name()) else { continue };
        let Ok(file_type) = solution.file_type() else {continue};
        if !file_type.is_file() {
            continue;
        }

        problems.push(problem);
    }
    let problems = problems;

    fs::write(
        Path::new(&manifest_dir).join("src").join("solution.rs"),
        gen_sol_module(&problems)?,
    )?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/solution");

    Ok(())
}

fn parse_number(file: impl AsRef<Path>) -> Option<u32> {
    let file: &Path = file.as_ref();
    let name = file.file_stem()?.to_str()?;
    let extension = file.extension()?;
    if extension != "rs" {
        return None;
    }

    if !name.starts_with("sol_") {
        return None;
    }

    Some(name.strip_prefix("sol_")?.parse().expect("solution number"))
}

/// Generates a solution glue module
fn gen_sol_module(problems: &[u32]) -> Result<String> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR to be set");
    let source_dir = Path::new(&manifest_dir)
        .join("src")
        .join("solution")
        .to_str()
        .unwrap()
        .to_owned();

    let modules: String = problems
        .iter()
        .map(|problem| {
            let hash = find_problem(*problem)
                .and_then(|problem| problem.hash)
                .map(|hash| <[u8; 16]>::from_hex(hash).expect("valid md5 hash string"));
            format!(
                "mod sol_{problem} {{
    #[path = \"{source_dir}/sol_{problem}.rs\"]
    mod sol;
    use sol::*;

    use md5::{{Md5, Digest}};

    /// Expected Hash of the Answer to Problem {problem}
    static EXPECTED_ANSWER_HASH: Option<[u8; 16]> = {hash:?};

    /// Check if the Hash of Solution {problem} matches the Expected Hash
    ///
    /// Autogenerated
    pub fn check_solution() -> color_eyre::Result<String> {{
        let result = format!(\"{{}}\", solution());
        let hash: [u8; 16] = {{
            let mut res = [0; 16];
            res.copy_from_slice(&Md5::new_with_prefix(&result).finalize()[..]);
            res
        }};
        if let Some(expected_hash) = EXPECTED_ANSWER_HASH {{
            if &expected_hash == &hash {{
                Ok(result)
            }} else {{
                Err(super::SolveError::WrongSolution(result).into())
            }}
        }} else {{
            Err(super::SolveError::NoHash{{ result, hash }}.into())
        }}
    }}
}}
"
            )
        })
        .collect();

    let match_arms = problems
        .iter()
        .map(|problem| format!("        {problem} => sol_{problem}::check_solution(),\n"))
        .collect::<String>();

    Ok(format!(
        "// Auto Generated Module compiling all solutions

use thiserror::Error;

/// Error while solving a Project Euler Problem
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum SolveError {{
    #[error(\"Database is missing the hash, result: {{result}}, hash: {{hash:?}}\")]
    NoHash {{ result: String, hash: [u8; 16] }},
    #[error(\"{{0}} did not match the hashed result\")]
    WrongSolution(String),
    #[error(\"There is no available solution for Problem {{0}}\")]
    NoSolution(u32),
}}

{modules}

/// Available Solutions
///
/// Autogenerated
pub static AVAILABLE_SOLUTIONS: &[u32] = &{problems:?};


/// Runs the solution to a particular problem
///
/// Autogenerated
pub fn solve(problem: u32) -> color_eyre::Result<String> {{
    match problem {{
{match_arms}
        id => Err(SolveError::NoSolution(id).into()),
    }}
}}
"
    ))
}
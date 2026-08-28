use std::process::ExitCode;

fn main() -> ExitCode {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("validate_solver_config: no files named");
        return ExitCode::from(2);
    }
    let mut status = 0;
    for path in &paths {
        match validate(path) {
            Ok(params) => println!("validate_solver_config: {path} ok ({params:?})"),
            Err(what) => {
                eprintln!("validate_solver_config: {path} REJECTED: {what}");
                status = 1;
            }
        }
    }
    if status == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn validate(path: &str) -> Result<pistol_solver::SolverParams, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{error}"))?;
    let file = pistol_solver::SolverConfigFile::parse(&text)?;
    file.validate().map_err(|error| format!("{error:?}"))
}

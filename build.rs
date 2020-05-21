use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR unset"));

    let postgres_a = run_build_sh(&manifest_dir)?;
    let path = PathBuf::from(postgres_a);

    println!("cargo:rustc-link-search={}", path.parent().unwrap().display());
    println!("cargo:rustc-link-lib=static=postgres");

    Ok(())
}

fn run_build_sh(pwd: &PathBuf) -> Result<String, std::io::Error> {
    run_command(Command::new(&format!("{}/build.sh", pwd.to_str().unwrap()))
        .current_dir(pwd))
}


fn run_command(mut command: &mut Command) -> Result<String, std::io::Error> {
    eprintln!("command={:?}", command);
    command = command
        .env("NUM_CPUS", num_cpus::get().to_string())
        .env_remove("DEBUG")
        .env_remove("MAKEFLAGS")
        .env_remove("MAKELEVEL")
        .env_remove("MFLAGS")
        .env_remove("DYLD_FALLBACK_LIBRARY_PATH")
        .env_remove("OPT_LEVEL")
        .env_remove("TARGET")
        .env_remove("PROFILE")
        .env_remove("OUT_DIR")
        .env_remove("HOST")
        .env_remove("NUM_JOBS");

    let output = command.output()?;

    let mut postgres_a = None;
    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            if line.starts_with("cargo:") {
                eprintln!("{}", line);
            } else {
                eprintln!("[stdout] {}", line);
            }

            // output postgres.o file is just the last line from stdout
            postgres_a = Some(line.to_owned());
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            eprintln!("[stderr] {}", line);
        }
    }

    if let None = postgres_a {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not determine location of generated 'postgres.a'"))
    } else if output.status.success() {
        Ok(postgres_a.unwrap())
    } else {
        Err(std::io::Error::last_os_error())
    }
}
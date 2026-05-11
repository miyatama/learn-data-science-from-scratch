mod vector;

use std::env;
use std::process;

fn parse_vector(s: &str) -> Result<Vec<f64>, String> {
    s.split(',')
        .map(|x| {
            x.trim()
                .parse::<f64>()
                .map_err(|_| format!("invalid number: '{}'", x))
        })
        .collect()
}

fn format_vector(v: &[f64]) -> String {
    let inner: Vec<String> = v
        .iter()
        .map(|x| {
            if x.fract() == 0.0 {
                format!("{}", *x as i64)
            } else {
                format!("{}", x)
            }
        })
        .collect();
    format!("[{}]", inner.join(", "))
}

fn format_scalar(x: f64) -> String {
    if x.fract() == 0.0 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}

fn is_help(args: &[String]) -> bool {
    args.iter().any(|a| a == "-h" || a == "--help")
}

fn print_global_help() {
    println!(
        "linear-algebra 1.0.1
Linear algebra vector operations

USAGE:
    linear-algebra <COMMAND> [OPTIONS]

COMMANDS:
    add        Add two vectors
    sub        Subtract two vectors
    scale      Multiply a vector by a scalar
    dot        Compute dot product of two vectors
    sumsq      Compute sum of squares of a vector
    magnitude  Compute magnitude of a vector

OPTIONS:
    -h, --help  Print help information"
    );
}

fn print_command_help(cmd: &str) {
    match cmd {
        "add" => println!(
            "linear-algebra-add
Add two vectors

USAGE:
    linear-algebra add <vector1> <vector2>

ARGS:
    <vector1>  First vector (comma-separated, e.g. 1,2,3)
    <vector2>  Second vector (comma-separated, e.g. 4,5,6)

OPTIONS:
    -h, --help  Print help information"
        ),
        "sub" => println!(
            "linear-algebra-sub
Subtract two vectors

USAGE:
    linear-algebra sub <vector1> <vector2>

ARGS:
    <vector1>  First vector (comma-separated, e.g. 5,7,9)
    <vector2>  Second vector (comma-separated, e.g. 4,5,6)

OPTIONS:
    -h, --help  Print help information"
        ),
        "scale" => println!(
            "linear-algebra-scale
Multiply a vector by a scalar

USAGE:
    linear-algebra scale <scalar> <vector>

ARGS:
    <scalar>  Scalar value (e.g. 2)
    <vector>  Vector (comma-separated, e.g. 1,2,3)

OPTIONS:
    -h, --help  Print help information"
        ),
        "dot" => println!(
            "linear-algebra-dot
Compute dot product of two vectors

USAGE:
    linear-algebra dot <vector1> <vector2>

ARGS:
    <vector1>  First vector (comma-separated, e.g. 1,2,3)
    <vector2>  Second vector (comma-separated, e.g. 4,5,6)

OPTIONS:
    -h, --help  Print help information"
        ),
        "sumsq" => println!(
            "linear-algebra-sumsq
Compute sum of squares of a vector

USAGE:
    linear-algebra sumsq <vector>

ARGS:
    <vector>  Vector (comma-separated, e.g. 1,2,3)

OPTIONS:
    -h, --help  Print help information"
        ),
        "magnitude" => println!(
            "linear-algebra-magnitude
Compute magnitude of a vector

USAGE:
    linear-algebra magnitude <vector>

ARGS:
    <vector>  Vector (comma-separated, e.g. 3,4)

OPTIONS:
    -h, --help  Print help information"
        ),
        _ => print_global_help(),
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_global_help();
        return Ok(());
    }

    if args.len() == 2 && is_help(&args[1..2]) {
        print_global_help();
        return Ok(());
    }

    let cmd = args[1].as_str();

    if is_help(&args[2..]) {
        print_command_help(cmd);
        return Ok(());
    }

    match cmd {
        "add" => {
            if args.len() != 4 {
                return Err("usage: add <vector1> <vector2>".to_string());
            }
            let v1 = parse_vector(&args[2])?;
            let v2 = parse_vector(&args[3])?;
            let result = vector::vector_add(&v1, &v2)?;
            println!("{}", format_vector(&result));
        }
        "sub" => {
            if args.len() != 4 {
                return Err("usage: sub <vector1> <vector2>".to_string());
            }
            let v1 = parse_vector(&args[2])?;
            let v2 = parse_vector(&args[3])?;
            let result = vector::vector_subtract(&v1, &v2)?;
            println!("{}", format_vector(&result));
        }
        "scale" => {
            if args.len() != 4 {
                return Err("usage: scale <scalar> <vector>".to_string());
            }
            let scalar = args[2]
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("invalid scalar: '{}'", args[2]))?;
            let v = parse_vector(&args[3])?;
            let result = vector::scalar_multiply(scalar, &v);
            println!("{}", format_vector(&result));
        }
        "dot" => {
            if args.len() != 4 {
                return Err("usage: dot <vector1> <vector2>".to_string());
            }
            let v1 = parse_vector(&args[2])?;
            let v2 = parse_vector(&args[3])?;
            let result = vector::dot_product(&v1, &v2)?;
            println!("{}", format_scalar(result));
        }
        "sumsq" => {
            if args.len() != 3 {
                return Err("usage: sumsq <vector>".to_string());
            }
            let v = parse_vector(&args[2])?;
            let result = vector::sum_of_squares(&v);
            println!("{}", format_scalar(result));
        }
        "magnitude" => {
            if args.len() != 3 {
                return Err("usage: magnitude <vector>".to_string());
            }
            let v = parse_vector(&args[2])?;
            let result = vector::magnitude(&v);
            println!("{}", format_scalar(result));
        }
        _ => {
            return Err(format!("unknown command: '{}'", cmd));
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

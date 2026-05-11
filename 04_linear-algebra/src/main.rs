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

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("usage: linear-algebra <command> [args...]".to_string());
    }

    match args[1].as_str() {
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
        cmd => {
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

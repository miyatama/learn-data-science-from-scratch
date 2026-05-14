mod correlation;
mod statistics;

use serde::Deserialize;
use std::{env, fs, process};

#[derive(Deserialize)]
struct Input {
    datas: Vec<Vec<f64>>,
}

fn print_help() {
    println!("統計計算 CLI - Data Science From Scratch Chapter 5");
    println!();
    println!("Usage:");
    println!("    statistics <input.json>");
    println!("    statistics -h | --help");
    println!();
    println!("Options:");
    println!("    <input.json>    統計計算対象の JSON ファイルパス");
    println!("    -h, --help      このヘルプを表示して終了");
    println!();
    println!("Input JSON Format:");
    println!("    {{");
    println!("      \"datas\": [");
    println!("        [<f64>, ...],   // datas[0]: 代表値・散らばりの計算対象");
    println!("        [<f64>, ...]    // datas[1]: 相関計算の対象（省略可）");
    println!("      ]");
    println!("    }}");
    println!();
    println!("Output:");
    println!("    代表値（平均値・中央値・25%分位数・75%分位数・最頻値）");
    println!("    散らばり（範囲・分散・標準偏差）");
    println!("    相関（共分散・相関係数）※ datas[1] がある場合のみ");
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        process::exit(0);
    }

    if args.len() < 2 {
        eprintln!("Usage: {} <input.json>", args[0]);
        eprintln!("Try '{} --help' for more information.", args[0]);
        process::exit(1);
    }

    let content = fs::read_to_string(&args[1])?;
    let input: Input = serde_json::from_str(&content)?;

    if input.datas.is_empty() {
        eprintln!("Error: datas フィールドが空です");
        process::exit(1);
    }

    let x = &input.datas[0];

    println!("=== 代表値 (datas[0]) ===");
    println!("平均値      : {:.4}", statistics::mean(x));
    println!("中央値      : {:.4}", statistics::median(x));
    println!("25%分位数   : {:.4}", statistics::quantile(x, 0.25));
    println!("75%分位数   : {:.4}", statistics::quantile(x, 0.75));
    println!("最頻値      : {:.4}", statistics::mode(x));

    println!();
    println!("=== 散らばり (datas[0]) ===");
    println!("最小値      : {:.4}", statistics::min(x));
    println!("最大値      : {:.4}", statistics::max(x));
    println!("数値の範囲  : {:.4}", statistics::data_range(x));
    println!("分散        : {:.4}", statistics::variance(x));
    println!("標準偏差    : {:.4}", statistics::std_deviation(x));

    if input.datas.len() >= 2 {
        let y = &input.datas[1];
        let min_len = x.len().min(y.len());
        let x_trim = &x[..min_len];
        let y_trim = &y[..min_len];

        let m = statistics::mean(x_trim);
        let s = statistics::std_deviation(x_trim);
        let lower = m - 3.0 * s;
        let upper = m + 3.0 * s;
        let (x3, y3): (Vec<f64>, Vec<f64>) = x_trim
            .iter()
            .zip(y_trim.iter())
            .filter(|(&xi, _)| xi >= lower && xi <= upper)
            .map(|(&xi, &yi)| (xi, yi))
            .unzip();

        println!();
        println!("=== 相関 (datas[0] vs datas[1]) ===");
        println!("共分散      : {:.4}", correlation::covariance(x_trim, y_trim));
        println!("相関係数    : {:.4}", correlation::correlation(x_trim, y_trim));
        println!("--- 3σ範囲内 ---");
        println!("共分散      : {:.4}", correlation::covariance(&x3, &y3));
        println!("相関係数    : {:.4}", correlation::correlation(&x3, &y3));
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

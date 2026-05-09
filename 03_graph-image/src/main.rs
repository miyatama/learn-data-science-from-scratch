mod input;
mod parser;
mod renderer;

use clap::Parser;
use renderer::{Renderer, bar::BarRenderer, line::LineRenderer, scatter::ScatterRenderer};

#[derive(Parser)]
#[command(name = "graph-image", about = "Generate graph images from JSON")]
struct Cli {
    #[arg(short, long, default_value = "./output/image.png")]
    output: String,

    #[arg(short, long, default_value_t = 256)]
    size: u32,

    #[arg(short, long)]
    input: Option<String>,

    #[arg(short = 't', long = "type", default_value = "line")]
    graph_type: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let json = input::read(cli.input.as_deref())?;

    let renderer: Box<dyn Renderer> = match cli.graph_type.as_str() {
        "line" => Box::new(LineRenderer {
            data: parser::parse_line(&json)?,
        }),
        "bar" => Box::new(BarRenderer {
            data: parser::parse_bar(&json)?,
        }),
        "scatter" => Box::new(ScatterRenderer {
            data: parser::parse_scatter(&json)?,
        }),
        other => anyhow::bail!(
            "Unknown graph type: '{}'. Use line, bar, or scatter.",
            other
        ),
    };

    renderer.render(&cli.output, cli.size)?;
    Ok(())
}

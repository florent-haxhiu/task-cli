use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Task {
    #[arg(short, long, default_value_t = 1)]
    pub id: i32,
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub done: bool,
}

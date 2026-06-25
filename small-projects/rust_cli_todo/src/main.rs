use Rust_CLI_Todo::todo;
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A system level todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { task } => {
            todo::add_task(task).expect("failed to add task ");
        }
        Commands::List => {
            todo::list().expect("failed to list tasks");
        }
        Commands::Complete { id } => {
            todo::complete(id).expect("failed to mark complete the task");
        }
        Commands::Delete { id } => {
            todo::delete(id).expect("failed to delete task");
        }
    }
}

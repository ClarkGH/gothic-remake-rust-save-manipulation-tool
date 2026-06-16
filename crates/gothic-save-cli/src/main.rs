use anyhow::Context;
use clap::{Parser, Subcommand};
use gothic_save::Save;

#[derive(Parser)]
#[command(name = "gothic-save", about = "Modify Gothic 1 Remake save files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show basic info about a save file
    Info {
        /// Path to the .sav (or whatever extension the game uses)
        path: std::path::PathBuf,
    },
    // Future: Edit, Dump, Patch, etc.
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { path } => {
            let save = Save::from_path(&path)
                .with_context(|| format!("reading save at {}", path.display()))?;
            println!("Save size: {} bytes", save.raw.len());
        }
    }

    Ok(())
}

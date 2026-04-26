use anyhow::Result;

pub fn main() -> Result<()> {
    println!("The Fuck - Update Instructions");
    println!();

    println!("Cargo (built from source):");
    println!("  cargo install --path thefuck --force");
    println!("  # or from the repo root:");
    println!("  cd thefuck && cargo install --path . --force");
    println!();

    println!("Homebrew (macOS/Linux):");
    println!("  brew upgrade thefuck");
    println!();

    println!("APT (Ubuntu/Debian):");
    println!("  sudo apt update && sudo apt upgrade thefuck");
    println!();

    println!("Pacman (Arch Linux):");
    println!("  sudo pacman -Syu thefuck");
    println!();

    println!("Windows (PowerShell):");
    println!("  irm https://github.com/HyShmily/thefuck-upgrade/raw/main/install.ps1 | iex");
    println!();

    println!("Check the latest release at:");
    println!("  https://github.com/HyShmily/thefuck-upgrade/releases");

    Ok(())
}

use anyhow::Result;
use std::fs;

pub fn main() -> Result<()> {
    println!("The Fuck - Uninstall");
    println!();

    // Remove history and data directory
    if let Some(base) = dirs::data_local_dir().or_else(dirs::home_dir) {
        let data_dir = base.join("thefuck");
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir)?;
            println!("Removed data directory: {}", data_dir.display());
        } else {
            println!("No data directory found at: {}", data_dir.display());
        }
    }

    println!();
    println!("To complete uninstallation, remove the shell alias from your config:");
    println!();

    #[cfg(unix)]
    {
        println!("Remove this line from ~/.bashrc or ~/.zshrc:");
        println!("  eval \"$(thefuck --alias)\"");
        println!();
        println!("Then uninstall the binary:");
        println!("  cargo uninstall thefuck");
        println!("  # or if installed via package manager:");
        println!("  brew uninstall thefuck       # Homebrew");
        println!("  sudo apt remove thefuck      # APT");
        println!("  sudo pacman -R thefuck       # Pacman");
    }

    #[cfg(windows)]
    {
        println!("Remove the Invoke-Fuck function and alias from your PowerShell profile:");
        println!("  (Check $PROFILE for the function definition)");
        println!();
        println!("Then uninstall the binary:");
        println!("  cargo uninstall thefuck");
    }

    println!();
    println!("Goodbye!");

    Ok(())
}

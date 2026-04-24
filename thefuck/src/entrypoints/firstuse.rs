use anyhow::Result;

pub fn main() -> Result<()> {
    println!("The Fuck - Command Corrector");
    println!();
    println!("Thank you for installing The Fuck!");
    println!();
    println!("To get started, add the following alias to your shell configuration:");
    println!();

    #[cfg(unix)]
    {
        println!("For bash/zsh:");
        println!("  eval $(thefuck --alias)");
    }

    #[cfg(windows)]
    {
        println!("For PowerShell:");
        println!("  Invoke-Expression (thefuck --alias | Out-String)");
    }

    println!();
    println!("After adding the alias, restart your shell or run:");
    println!("  source ~/.bashrc  # for bash");
    println!("  source ~/.zshrc   # for zsh");
    println!("  # For PowerShell, restart PowerShell");
    println!();
    println!("Then try running a command with a typo:");
    println!("  gti status");
    println!("  fuck");
    println!();

    Ok(())
}

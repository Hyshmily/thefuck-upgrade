pub fn print_alias_command() {
    #[cfg(unix)]
    {
        println!("alias fuck='TF_HISTORY=$(fc -ln -1) thefuck'");
    }

    #[cfg(windows)]
    {
        println!(
            "function Invoke-Fuck {{
    $cmd = Get-History -Count 1 | Select-Object -ExpandProperty CommandLine
    $env:TF_HISTORY = $cmd
    thefuck
}}
Set-Alias -Name fuck -Value Invoke-Fuck"
        );
    }
}

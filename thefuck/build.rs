use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let rules_path = Path::new(&out_dir).join("rules.txt");

    // Collect all rule files
    let mut rules = Vec::new();
    for entry in fs::read_dir("src/rules")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Some(name) = path.file_stem() {
                rules.push(name.to_string_lossy().into_owned());
            }
        }
    }

    // Write rules list to build output
    fs::write(&rules_path, rules.join("\n"))?;
    println!("cargo:rerun-if-changed=src/rules");

    Ok(())
}

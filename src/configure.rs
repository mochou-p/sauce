// mochou-p/sauce/src/configure.rs

pub fn command(mut args: impl Iterator<Item = String>) {
    let subcommands = "system";

    let Some(word) = args.next() else {
        return eprintln!("missing configuration target: \x1b[36m{subcommands}\x1b[0m");
    };

    match word.as_str() {
        "system" => configure(format!("{}{}", super::ROOT, super::SYSTEM_CONFIG)),
        other    => eprintln!("unknown configure target:  \x1b[31m`{other}`\x1b[0m\n  known configure targets: \x1b[32m{subcommands}\x1b[0m")
    }
}

fn configure(filepath: String) {
    if !matches!(std::fs::exists(&filepath), Ok(true)) {
        return eprintln!("configuration file `{filepath}` does not exist or is not accessible");
    }

    let Ok(editor) = std::env::var("EDITOR") else {
        return eprintln!("environment variable `EDITOR` is not set");
    };

    std::process::Command::new(editor).arg(filepath).status().unwrap();
}


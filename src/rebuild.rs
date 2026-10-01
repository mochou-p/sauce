// mochou-p/sauce/src/rebuild.rs

pub fn command(mut args: impl Iterator<Item = String>) {
    let subcommands = "system";

    let Some(word) = args.next() else {
        return eprintln!("missing rebuild target: \x1b[36m{subcommands}\x1b[0m");
    };

    match word.as_str() {
        "system" => rebuild(format!("{}{}", super::ROOT, super::SYSTEM_CONFIG)).unwrap(),
        other    => eprintln!("unknown rebuild target:  \x1b[31m`{other}`\x1b[0m\n  known rebuild targets: \x1b[32m{subcommands}\x1b[0m")
    }
}

fn rebuild(filepath: String) -> mlua::Result<()> {
    let config_path = std::path::PathBuf::from(filepath);

    let Ok(config_contents) = std::fs::read_to_string(&config_path) else {
        return Ok(eprintln!("failed to read file `{}`", config_path.display()));
    };

    let lua  = super::lua::setup()?;
    let jobs = super::lua::parse(&lua, config_contents)?;

    // TODO: run in parallel
    for (package_name, install, post_install) in jobs {
        // TODO: check checksum first

        println!("\x1b[104m  \x1b[0m {package_name}: running install script..");

        if let Err(err) = install.call::<()>(()) {
            eprintln!("\x1b[101m  \x1b[0m {package_name}: install script failed:\n\x1b[31m{err}\x1b[0m");
            continue;
        }

        println!("\x1b[104m  \x1b[0m {package_name}: running post install script..");

        if let Err(err) = post_install.call::<()>(()) {
            eprintln!("\x1b[101m  \x1b[0m {package_name}: post install script failed:\n\x1b[31m{err}\x1b[0m");
            continue;
        }

        println!("\x1b[102m  \x1b[0m {package_name}");
    }

    Ok(())
}


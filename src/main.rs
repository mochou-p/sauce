// mochou-p/sauce/src/main.rs

#![feature(str_as_str)]


#[cfg(debug_assertions)]
const ROOT: &str = "./root/";
#[cfg(not(debug_assertions))]
const ROOT: &str = "/";

const SYSTEM_CONFIG: &str = "etc/sauce.lua";

fn main() {
    command(std::env::args().skip(1));
}

fn command(mut args: impl Iterator<Item = String>) {
    let subcommands = "help | configure | rebuild";

    let Some(word) = args.next() else {
        return eprintln!("missing command: \x1b[36m{subcommands}\x1b[0m");
    };

    match word.as_str() {
        "help"      => cmd_help(),
        "configure" => cmd_configure(args),
        "rebuild"   => cmd_rebuild(args),
        other       => eprintln!("unknown command:  \x1b[31m`{other}`\x1b[0m\n  known commands: \x1b[32m{subcommands}\x1b[0m")
    }
}

fn cmd_help() {
    println!("{} - {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
}

fn cmd_configure(mut args: impl Iterator<Item = String>) {
    let subcommands = "system";

    let Some(word) = args.next() else {
        return eprintln!("missing configuration target: \x1b[36m{subcommands}\x1b[0m");
    };

    match word.as_str() {
        "system" => configure(format!("{ROOT}{SYSTEM_CONFIG}")),
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

fn cmd_rebuild(mut args: impl Iterator<Item = String>) {
    let subcommands = "system";

    let Some(word) = args.next() else {
        return eprintln!("missing rebuild target: \x1b[36m{subcommands}\x1b[0m");
    };

    match word.as_str() {
        "system" => rebuild(format!("{ROOT}{SYSTEM_CONFIG}")).unwrap(),
        other    => eprintln!("unknown rebuild target:  \x1b[31m`{other}`\x1b[0m\n  known rebuild targets: \x1b[32m{subcommands}\x1b[0m")
    }
}

fn rebuild(filepath: String) -> mlua::Result<()> {
    let config_path = std::path::PathBuf::from(filepath);

    let Ok(config_contents) = std::fs::read_to_string(&config_path) else {
        return Ok(eprintln!("failed to read file `{}`", config_path.display()));
    };

    let     lua    = setup_lua()?;
    let     config = lua.load(config_contents).eval::<mlua::Table>()?;
    let mut jobs   = vec![];

    for package in config.pairs::<String, mlua::Table>() {
        let (package_name, package_table) = package?;

        // TODO: this is a bad way of doing this, and at a bad place :D
        if matches!(std::fs::exists(format!("{ROOT}{}/store/{package_name}/", env!("CARGO_BIN_NAME"))), Ok(true)) {
            println!("skipped {package_name}");
            continue;
        }

        let options = match package_table.get("source")? {
            mlua::Value::String(string) => lua.load(string.to_str()?.as_str()).eval()?,
            mlua::Value::Table(table)   => table,
            _                           => panic!()
        };

        let installer = options.get::<mlua::Function>("install_script")?;
        jobs.push((package_name, installer));
    }

    // TODO: run in parallel
    for (package_name, installer) in jobs {
        // TODO: check checksum first

        println!("installing {package_name}..");
        installer.call::<()>(())?;
        println!("installed  {package_name}");
    }

    Ok(())
}

fn setup_lua() -> mlua::Result<mlua::Lua> {
    let lua     = mlua::Lua::new();
    let globals = lua.globals();
    let utils   = lua.create_table()?;
    let apps    = lua.load(include_str!("../repo/apps.lua")).eval::<mlua::Table>()?;

    let fn_github     = lua.create_function(github    )?;
    let fn_download   = lua.create_function(download  )?;
    let fn_unpack_zip = lua.create_function(unpack_zip)?;

    utils.set("github",     fn_github    )?;
    utils.set("download",   fn_download  )?;
    utils.set("unpack_zip", fn_unpack_zip)?;

    globals.set("apps",  apps )?;
    globals.set("utils", utils)?;

    Ok(lua)
}

fn github(_: &mlua::Lua, (owner, repo, tag, release): (String, String, String, String)) -> mlua::Result<String> {
    Ok(format!("https://github.com/{owner}/{repo}/releases/download/{tag}/{release}"))
}

fn download(lua: &mlua::Lua, url: String) -> mlua::Result<mlua::AnyUserData> {
    let response = reqwest::blocking::get(url).unwrap().error_for_status().unwrap();

    lua.create_any_userdata(response)
}

fn unpack_zip(_: &mlua::Lua, (repo, tag, data): (String, String, mlua::AnyUserData)) -> mlua::Result<()> {
    use bytes::buf::Buf as _;

    let     response      = data.take::<reqwest::blocking::Response>()?;
    let mut reader        = response.bytes().unwrap().reader();
    let     prefix_string = format!("{ROOT}{}/store/{repo}/{tag}", env!("CARGO_BIN_NAME"));
    let     prefix        = std::path::Path::new(&prefix_string);

    while let Some(mut entry) = zip::read::read_zipfile_from_stream(&mut reader).unwrap() {
        let pathbuf = prefix.join(entry.enclosed_name().unwrap());

        if entry.is_dir() {
            std::fs::create_dir_all(pathbuf).unwrap();
        } else if entry.is_file() {
            let mut file = std::fs::File::create(pathbuf).unwrap();
            std::io::copy(&mut entry, &mut file).unwrap();
        } else {
            eprintln!("{} is not a dir/file", pathbuf.display());
        }
    }

    Ok(())
}


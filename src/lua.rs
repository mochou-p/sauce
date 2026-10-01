// mochou-p/sauce/src/lua.rs

pub fn setup() -> mlua::Result<mlua::Lua> {
    let lua     = mlua::Lua::new();
    let globals = lua.globals();
    let utils   = lua.create_table()?;
    let apps    = lua.load(include_str!("../repo/apps.lua")).eval::<mlua::Table>()?;

    utils.set("github",        lua.create_function(super::ffi::github       )?)?;
    utils.set("download",      lua.create_function(super::ffi::download     )?)?;
    utils.set("unpack_zip",    lua.create_function(super::ffi::unpack_zip   )?)?;
    utils.set("unpack_tar_gz", lua.create_function(super::ffi::unpack_tar_gz)?)?;

    globals.set("utils", utils)?;
    globals.set("apps",  apps )?;

    Ok(lua)
}

pub fn parse(lua: &mlua::Lua, config_contents: String) -> mlua::Result<Vec<(String, mlua::Function, mlua::Function)>> {
    let     config = lua.load(config_contents).eval::<mlua::Table>()?;
    let mut jobs   = vec![];

    for package in config.pairs::<String, mlua::Table>() {
        let (package_name, package_table) = package?;

        // TODO: this is a bad way of doing this, and at a bad place :D
        if matches!(std::fs::exists(format!("{}{}/store/{package_name}/", super::ROOT, env!("CARGO_BIN_NAME"))), Ok(true)) {
            println!("\x1b[107m  \x1b[0m skipped {package_name}");
            continue;
        }

        let options = match package_table.get("source")? {
            mlua::Value::String(string) => lua.load(string.to_str()?.as_str()).eval()?,
            mlua::Value::Table(table)   => table,
            other                       => {
                return Err(mlua::Error::ExternalError(std::sync::Arc::new(
                    ParseError::InvalidPackageSource(package_name, other)
                )));
            }
        };

        let      installer = options.get::<mlua::Function>(     "install_script")?;
        let post_installer = options.get::<mlua::Function>("post_install_script")?;

        jobs.push((package_name, installer, post_installer));
    }

    Ok(jobs)
}

#[derive(Debug)]
enum ParseError {
    InvalidPackageSource(String, mlua::Value)
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPackageSource(name, source) => write!(f, "invalid {name} source: {source:?}")
        }
    }
}

impl std::error::Error for ParseError {}


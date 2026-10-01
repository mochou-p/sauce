// mochou-p/sauce/src/ffi.rs

// NOTE: all lua files can use these rust functions via the global `utils` table,
//       the main purpose of this is to use them in package install scripts


pub fn github(_lua: &mlua::Lua, (owner, repo, tag, release): (String, String, String, String)) -> mlua::Result<String> {
    Ok(format!("https://github.com/{owner}/{repo}/releases/download/{tag}/{release}"))
}

pub fn download(lua: &mlua::Lua, url: String) -> mlua::Result<mlua::AnyUserData> {
    let response = reqwest::blocking::get(url).unwrap().error_for_status().unwrap();

    lua.create_any_userdata(response)
}

pub fn unpack_zip(_lua: &mlua::Lua, (repo, tag, data): (String, String, mlua::AnyUserData)) -> mlua::Result<()> {
    use bytes::buf::Buf as _;

    let     response      = data.take::<reqwest::blocking::Response>()?;
    let mut reader        = response.bytes().unwrap().reader();
    let     prefix_string = format!("{}{}/store/{repo}/{tag}", super::ROOT, env!("CARGO_BIN_NAME"));
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

pub fn unpack_tar_gz(_lua: &mlua::Lua, (repo, tag, data): (String, String, mlua::AnyUserData)) -> mlua::Result<()> {
    use bytes::buf::Buf as _;

    let response      = data.take::<reqwest::blocking::Response>()?;
    let reader        = response.bytes().unwrap().reader();
    let prefix_string = format!("{}{}/store/{repo}/{tag}", super::ROOT, env!("CARGO_BIN_NAME"));
    let prefix        = std::path::Path::new(&prefix_string);

    let     decoder = flate2::read::GzDecoder::new(reader);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(prefix).unwrap();

    Ok(())
}


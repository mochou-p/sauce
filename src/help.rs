// mochou-p/sauce/src/help.rs

pub fn command() {
    println!(
        concat!(
            #[cfg(debug_assertions)]
            "[DEBUG VERSION]\n",
            env!("CARGO_BIN_NAME"), " - ", env!("CARGO_PKG_VERSION"), '\n',
            '\n',
            "help                print this message\n",
            "configure system    open `{}{}` with $EDITOR\n",
            "rebuild   system    apply the system config\n",
            "clean               clean the `{}", env!("CARGO_BIN_NAME"), "` dir"
        ),
        super::ROOT, super::SYSTEM_CONFIG,
        super::ROOT
    );
}


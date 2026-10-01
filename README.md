<!-- mochou-p/sauce/README.md -->

# sauce
a declarative package manager, which uses [lua](https://github.com/lua/lua) for configuration.
this means, you do not run traditional "install commands" in your terminal to get packages,
but rather just edit your config file. this gives you incredibly convenient reuse and sharing possibilities!

> [!NOTE]
> - this package manager only supports x86_64 linux, except NixOS and similar distros  
> - in debug mode, the local [root/](root) directory is used instead of your actual filesystem root  

## capabilities
there are two layers of support for packages: remote host, archive format
- supported remote hosts: github releases
- supported archive formats: `.tar.gz`, `.zip`

## contributing
adding a new package is simple, look at existing packages in [repo/apps.lua](repo/apps.lua)  
inspect [src/ffi.rs](src/ffi.rs) for useful methods on the global `utils` table  

## usage
a system config file can look like this:
```lua
-- /etc/sauce.lua

-- return a table of packages
return {
    -- get the `yazi` package
    yazi = {
        -- use the latest available version
        source = apps.yazi.latest,
        -- or you can pin an exact version
        -- by doing e.g. `apps.yazi["v26.9.1"]`
    },
}
```
you can edit this by running `sauce configure system`  
and then apply it with `sauce rebuild system`  

see `sauce help` for more information on commands  

## available packages
### apps
[tmux](https://github.com/tmux/tmux)
[yazi](https://github.com/sxyazi/yazi)
### libs
none yet


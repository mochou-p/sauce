-- mochou-p/sauce/repo/apps.lua

return {
    yazi = {
        latest = [=[apps.yazi["v26.9.1"]]=],

        ["v26.9.1"] = {
            install_script = function()
                local owner   = "sxyazi"
                local repo    = "yazi"
                local tag     = "v26.9.1"
                local release = "yazi-x86_64-unknown-linux-gnu.zip"

                local url     = utils.github(owner, repo, tag, release)
                local archive = utils.download(url)

                utils.unpack_zip(repo, tag, archive)
            end,
        },
    },
}


-- mochou-p/sauce/repo/apps.lua

return {
    tmux = {
        latest = [=[apps.tmux["3.7c"]]=],

        -- https://github.com/tmux/tmux/releases/download/3.7c/tmux-3.7c.tar.gz
        ["3.7c"] = {
            install_script = function()
                local owner   = "tmux"
                local repo    = "tmux"
                local tag     = "3.7c"
                local release = "tmux-3.7c.tar.gz"

                local url     = utils.github(owner, repo, tag, release)
                local archive = utils.download(url)

                utils.unpack_tar_gz(repo, tag, archive)
            end,

            post_install_script = function()
            end,
        },
    },

    yazi = {
        latest = [=[apps.yazi["v26.9.1"]]=],

        -- https://github.com/sxyazi/yazi/releases/download/v26.9.1/yazi-x86_64-unknown-linux-gnu.zip
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

            post_install_script = function()
            end,
        },
    },
}


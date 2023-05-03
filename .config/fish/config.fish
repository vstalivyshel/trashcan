if status is-interactive
    fish_add_path -Ua $HOME/.cargo/bin $HOME/.local/bin $HOME/.local/scr
    fish_add_path /home/linuxbrew/.linuxbrew/opt/clang-format/bin

	set -xga PKG_CONFIG_PATH /usr/lib64/pkgconfig
    set fish_cursor_default block
    set -Ux EDITOR kak
    set -Ux VISUAL $EDITOR

    bind \cv $EDITOR
    bind \cl 'ls -Ahls; commandline -f repaint'
    setxkbmap -option "ctrl:nocaps"

    set -g FZF_DEFAULT_COMMAND fd --hidden
    set -g PYTHONDONTWRITEBYTECODE 1

    alias up='sudo dnf update -y; rustup update; brew upgrade'
    alias dfs="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME"
    alias dfsdo='dfs commit -a -m "yea"; dfs push origin main'

    eval (/home/linuxbrew/.linuxbrew/bin/brew shellenv)
end

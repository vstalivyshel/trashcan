if status is-interactive
    fish_add_path /usr/local/texlive/2023/bin/x86_64-linux
    set -xga PKG_CONFIG_PATH /usr/lib64/pkgconfig /usr/lib/x86_64-linux-gnu/pkgconfig 
    set fish_cursor_default block
    set -Ux EDITOR kak
    set -Ux VISUAL $EDITOR

    bind \cv $EDITOR
    bind \cl 'ls -Ahls; commandline -f repaint'
    setxkbmap -option "ctrl:nocaps"

    set -g PYTHONDONTWRITEBYTECODE 1

    alias up='sudo dnf update -y; rustup update; brew upgrade'
    alias dfs="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME"
    alias dfsdo='dfs commit -a -m "yea"; dfs push origin main'

    eval (/home/linuxbrew/.linuxbrew/bin/brew shellenv)
end

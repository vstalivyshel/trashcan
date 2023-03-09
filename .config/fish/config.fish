if status is-interactive
    fish_add_path -Ua $HOME/.cargo/bin $HOME/.local/bin $HOME/.local/scr $HOME/.nimble/bin

    set fish_cursor_default block
    set -Ux EDITOR kak
    set -Ux VISUAL $EDITOR

    bind \cv $EDITOR
    bind \cl 'ls -Ahls; commandline -f repaint'
    setxkbmap -option "ctrl:nocaps"

    set -U FZF_DEFAULT_COMMAND fd --hidden
    set -U PYTHONDONTWRITEBYTECODE 1

    alias up='sudo dnf update -y && rustup update'
    alias dfs="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME"
    alias dfss="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME status"
    alias dfsdo='dfs commit -a -m "yea"; dfs push origin main'

    eval (/home/linuxbrew/.linuxbrew/bin/brew shellenv)
end

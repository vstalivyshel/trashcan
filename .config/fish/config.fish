if status is-interactive
    fish_add_path -Ua $HOME/.cargo/bin $HOME/.local/bin

    set -Ux EDITOR $HOME/.cargo/bin/hx    
    set -Ux VISUAL $EDITOR

    bind \cv hx
    bind \cl 'ls -Ahls; commandline -f repaint'
    setxkbmap -option "ctrl:nocaps"

    set -g PYTHONDONTWRITEBYTECODE 1

    alias up='sudo dnf update -y && rustup update'
    alias dfs="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME"
    alias dfss="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME status"
    alias dfsdo='dfs commit -a -m "yea"; dfs push origin main'
end

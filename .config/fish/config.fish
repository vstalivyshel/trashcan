if status is-interactive
    fish_add_path /usr/local/texlive/2023/bin/x86_64-linux
    set -xga PKG_CONFIG_PATH /usr/lib64/pkgconfig /usr/lib/x86_64-linux-gnu/pkgconfig
    set -x KAKOUNE_POSIX_SHELL /bin/bash
    set fish_cursor_default block
    set -Ux EDITOR kak
    set -Ux VISUAL $EDITOR

    bind \cv 'goto'
    bind \cl 'ls -Ahls; commandline -f repaint'
    setxkbmap -option "ctrl:nocaps"

    # alias up='sudo apt update -y && sudo apt upgrade -y; sudo apt autoremove -y; rustup update'
    alias up='sudo dnf upgrade -y; rustup update'
    alias dfs="/usr/bin/git --git-dir=$HOME/.dots/ --work-tree=$HOME"
    alias dfsdo='dfs commit -a -m "yea"; dfs push origin main'
    alias feh='feh --keep-zoom-vp -d -B black'
end

# pnpm
set -gx PNPM_HOME "/home/vstalivyshel/.local/share/pnpm"
if not string match -q -- $PNPM_HOME $PATH
  set -gx PATH "$PNPM_HOME" $PATH
end
# pnpm end

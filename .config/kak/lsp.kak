evaluate-commands %sh{kak-lsp --kakoune -s $kak_session}

hook global WinSetOption filetype=(rust|python|lua|sh|c) %{
    lsp-enable-window
}

declare-option -hidden str modeline_progress ""
define-command -hidden -params 6 -override lsp-handle-progress %{
    set global modeline_progress %sh{
        if ! "$6"; then
            echo "$2${5:+" ($5%)"}${4:+": $4"}"
        fi
    }
}

set-option global lsp_hover_anchor true
set-option global lsp_hover_max_lines 0
set-option global lsp_insert_spaces true

set-option global modelinefmt "%opt{modeline_progress} %opt{modelinefmt} | %sh{date '+%A %D %T'}" 

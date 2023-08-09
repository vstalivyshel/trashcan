# https://github.com/krornus/kakoune-cargo/blob/master/cargo.kak
# https://gitlab.com/Screwtapello/kakoune-cargo/-/blob/master/cargo.kak?ref_type=heads

#######################
# Syntax highlighting #
#######################
add-highlighter shared/cargo group
add-highlighter shared/cargo/items regions
add-highlighter shared/cargo-share regions

# shared highlighters
# seperate region set in shared/ so it is not loaded by default
# these are used in both error and warning message region items
add-highlighter shared/cargo-share/rust region "^[0-9]+ \|" $ ref rust
add-highlighter shared/cargo-share/help region "^\s+\|" $ regions
add-highlighter shared/cargo-share/help/rust region "`" "`" ref rust
add-highlighter shared/cargo-share/help/info default-region group
add-highlighter shared/cargo-share/help/info/help regex "help" 0:default+b
add-highlighter shared/cargo-share/attribute region "#\[" \] ref rust

# error message
add-highlighter shared/cargo/items/error region "^error" "^\n" group
add-highlighter shared/cargo/items/error/context group
add-highlighter shared/cargo/items/error/error regex "^(error)(?:\[(E[0-9]+)\])?" 1:red+b 2:cyan
add-highlighter shared/cargo/items/error/arrow regex "(?S)(-->) (.+):([0-9]+):([0-9]+)" 1:red 2:default+b 3:cyan 4:cyan
add-highlighter shared/cargo/items/error/context/pointer regex "\s(-+|\^+)\s" 1:red+b
add-highlighter shared/cargo/items/error/context/share ref cargo-share

# warning message
add-highlighter shared/cargo/items/warning region "^(warning)" "^\n" group
add-highlighter shared/cargo/items/warning/context group
add-highlighter shared/cargo/items/warning/warning regex "^(warning)(?:\[(E[0-9]+)\])?" 1:yellow+b 2:cyan
add-highlighter shared/cargo/items/warning/arrow regex "(?S)(-->) (.+):([0-9]+):([0-9]+)" 1:yellow 2:default+b 3:cyan 4:cyan
add-highlighter shared/cargo/items/warning/context/pointer regex "\s(-+|\^+)\s" 1:yellow+b
add-highlighter shared/cargo/items/warning/context/share ref cargo-share

# finished message
add-highlighter shared/cargo/items/finished region "^\s+Finished dev" $ group
add-highlighter shared/cargo/items/finished/finished regex "Finished dev" 0:green+b
add-highlighter shared/cargo/items/finished/flags regions
add-highlighter shared/cargo/items/finished/flags/flags region \[ \] group
add-highlighter shared/cargo/items/finished/flags/flags/flag regex '[a-zA-Z0-9_\-]+' 0:cyan
add-highlighter shared/cargo/items/finished/seconds regex "in ([0-9]+\.[0-9]+)s" 1:default+b

# test result
add-highlighter shared/cargo/ok_test regex "ok$" 0:green+b
add-highlighter shared/cargo/failed_test regex "FAILED" 0:red+b

# global highlighters
add-highlighter shared/cargo/error regex "^(error):" 1:red+b
add-highlighter shared/cargo/compile regex "^\s+Compiling" 0:green+b
add-highlighter shared/cargo/check regex "^\s+Checking" 0:yellow
add-highlighter shared/cargo/lineno regex "^([0-9]+) (\|)" 1:cyan+b 2:default

declare-option -docstring "shell command run to build the project" \
    str cargocmd cargo

declare-option -docstring "regex describing cargo error references" \
    regex \
    cargo_error_pattern \
    "^\h*(?:error|warning|note)(?:\[[A-Z0-9]+\])?: ([^\n]*)\n *--> ([^\n]*?):(\d+)(?::(\d+))?"

declare-option -docstring "name of the client in which utilities display information" \
    str toolsclient
declare-option -docstring "name of the client in which all source code jumps will be executed" \
    str jumpclient
declare-option -hidden int cargo_current_error_line
declare-option -hidden str cargo_workspace_root

define-command -params .. \
-docstring %{cargo [<arguments>]: cargo utility wrapper
All the optional arguments are forwarded to the cargo utility} \
-shell-script-candidates %{ printf "test\ncheck\nbuild\nrun" } \
cargo %{
    evaluate-commands %sh{
        workspace_root=$(
            cargo metadata --format-version=1 |
            grep -o '"workspace_root":"[^"]*' |
            grep -o '[^"]*$'
        )
        quoted_workspace_root="'""$(
            printf %s "$workspace_root" |
            sed -e "s/'/''/g"
        )""/'"

        output=$(mktemp -d "${TMPDIR:-/tmp}"/kak-cargo.XXXXXXXX)/fifo
        mkfifo ${output}
        ( eval ${kak_opt_cargocmd} "$@" > ${output} 2>&1 ) > /dev/null 2>&1 < /dev/null &

        printf %s\\n "
            evaluate-commands -try-client '$kak_opt_toolsclient' %{
               edit! -fifo ${output} -scroll *cargo*
               set-option buffer filetype cargo
               set-option buffer cargo_current_error_line 1
               set-option buffer cargo_workspace_root $quoted_workspace_root
               hook -once buffer BufCloseFifo .* %{
                   nop %sh{ rm -r $(dirname ${output}) }
                   evaluate-commands -try-client '$kak_client' %{
                       echo -- Completed cargo $*
                   }
               }
           }
        "
    }
}

hook -group cargo-highlight global WinSetOption filetype=cargo %{
    add-highlighter window/cargo ref cargo
}

hook global WinSetOption filetype=cargo %{
    #hook buffer -group cargo-hooks NormalKey <ret> cargo-jump
    map -docstring "Jump to current error" buffer normal <ret> %{: cargo-jump<ret>}
}

hook -group cargo-highlight global WinSetOption filetype=(?!cargo).* %{
    remove-highlighter window/cargo
}

hook global WinSetOption filetype=(?!cargo).* %{
    #remove-hooks buffer cargo-hooks
    unmap buffer normal <ret> %{: cargo-jump<ret>}
}

declare-option -docstring "name of the client in which all source code jumps will be executed" \
    str jumpclient

define-command -hidden cargo-open-error -params 4 %{
    evaluate-commands -try-client %opt{jumpclient} %{
        edit -existing "%arg{1}" "%arg{2}" "%arg{3}"
        info -anchor "%arg{2}.%arg{3}" "%arg{4}"
        try %{ focus }
    }
}

define-command -hidden cargo-jump %{
    evaluate-commands %{
        # We may be in the middle of an error.
        # To find it, we search for the next error
        # (which definitely moves us past the end of this error)
        # and then search backward
        execute-keys "/" %opt{cargo_error_pattern} <ret>
        execute-keys <a-/> %opt{cargo_error_pattern} <ret><a-:> "<a-;>"

        # We found a Cargo error, let's open it.
        set-option buffer cargo_current_error_line "%val{cursor_line}"
        cargo-open-error \
            "%opt{cargo_workspace_root}%reg{2}" \
            "%reg{3}" \
            "%sh{ echo ${kak_main_reg_4:-1} }" \
            "%reg{1}"
    }
}

define-command cargo-next-error -docstring 'Jump to the next cargo error' %{
    try %{
        evaluate-commands -try-client %opt{jumpclient} %{
            buffer '*cargo*'
            execute-keys "%opt{cargo_current_error_line}gl" "/%opt{cargo_error_pattern}<ret>"
            cargo-jump
        }
        # Make sure the selected error is visible
        try %{
            evaluate-commands -client %opt{toolsclient} %{
                buffer '*cargo*'
                execute-keys %opt{cargo_current_error_line}gvv
            }
        }
    } catch %{
    	fail "No Cargo errors found"
    }
}

define-command cargo-previous-error -docstring 'Jump to the previous cargo error' %{
    try %{
        evaluate-commands -try-client %opt{jumpclient} %{
            buffer '*cargo*'
            execute-keys "%opt{cargo_current_error_line}gl" "<a-/>%opt{cargo_error_pattern}<ret>"
            cargo-jump
        }
        # Make sure the selected error is visible
        try %{
            evaluate-commands -client %opt{toolsclient} %{
                buffer '*cargo*'
                execute-keys %opt{cargo_current_error_line}gvv
            }
        }
    } catch %{
    	fail "No Cargo errors found"
    }
}

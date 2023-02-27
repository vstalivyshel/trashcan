define-command find-file \
    -docstring 'Fuzzy search for the files'\
    -override \
    %{
        prompt \
        -menu  \
        -shell-script-candidates %{fd -tf || find . -type f || return "sooqa"} \
        'file: ' \
        'edit %val{text}'
    }

define-command find-buffer \
	-docstring 'Open an exising buffer' \
	-override \
	%{prompt -menu -buffer-completion 'buffer: ' 'buffer %val{text}' }

define-command pwd \
	-docstring 'Echo the current working directory' \
	-override \
	%{ info %sh{ echo "$PWD/" } }
	
define-command sh \
	-override \
	-file-completion \
	-params 1.. \
	%{ echo %sh{ $@ } }

define-command set-filetype \
	-docstring 'Set a filetype for current buffer' \
	-override \
	-params 1 \
	%{ set-option current filetype %arg{1} }

define-command new-file \
	-docstring 'Create new file(s) with a given name'\
	-override \
	-params 1.. \
	%{ echo %sh{ touch $@ } edit %arg{1} }

alias global sef set-filetype
alias global nf new-file

define-command kak-talk \
	-docstring 'Send kak-command to all sessions' \
	-override \
	-command-completion \
	-params .. \
 	%{ nop %sh{ katalk $@ } }
alias global kak kak-talk 

define-command enable-auto-save \
	-docstring 'Enable auto-save' \
	-override \
    %{
        hook buffer -group autosave NormalIdle .* %{ try %{ write } }
        hook buffer -group autosave InsertIdle .* %{ try %{ write } }
    }

define-command disable-auto-save \
	-docstring 'Disable auto-save' \
	-override \
    %{ remove-hooks buffer autosave }

define-command disable-auto-save \
	-docstring 'Disable auto-save' \
	-override \
	%{ remove-hooks buffer auto-save }

define-command scratch \
	-docstring 'Open a scratch buffer' \
	-override \
	%{ buffer *scratch*  }

define-command find-session \
	-docstring 'Fuzzy search for the existing sessions' \
	-override \
	%{
    	prompt \
    	-menu \
    	-shell-script-candidates %{kak -l} \
    	'ses: ' \
    	'terminal kak -c %val{text}'
	}
	
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

define-command ls \
	-docstring 'List files in the current directory' \
	-override \
	%{ info -title "%val{client_env_PWD}/" %sh{ ls -Alh } }

define-command pwd \
	-docstring 'Echo the current working directory' \
	-override \
	%{ info %sh{ echo "$kak_buffile" } }

define-command sh \
	-override \
	-file-completion \
	-params 1.. \
	%{ echo %sh{ $@ } }

define-command lang \
	-docstring 'Set a filetype for current buffer' \
	-override \
	-params 1 \
	%{ set-option current filetype %arg{1} }

define-command new-file \
	-docstring 'Create new file(s) with a given name'\
	-override \
	-params 1.. \
	%{
    	echo %sh{ touch $@ }
    	edit %arg{1}
	}

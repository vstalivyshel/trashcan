# thanks to https://github.com/vbauerster/wezterm.kak/blob/master/rc/wezterm.kak

define-command wezterm \
	-docstring 'Run wezterm' \
	-override \
	-params .. \
	-shell-completion \
	%{ nop %sh{ WEZTERM_PANE=$kak_client_env_WEZTERM_PANE wezterm "$@" }  }

define-command wezterm-tab \
	-docstring 'Open a new wezterm tab' \
	-override \
	-shell-completion \
	-params .. \
	%{ wezterm cli spawn --cwd %val{client_env_PWD} -- %arg{@} }

define-command wezterm-vsplit \
	-docstring 'Vertical split with the given arguments' \
	-override \
	-shell-completion \
	-params .. \
	%{ wezterm cli split-pane --right --cwd %val{client_env_PWD} -- %arg{@} }

define-command wezterm-split \
	-docstring 'Horizontal split with the given arguments' \
	-override \
	-shell-completion \
	-params .. \
	%{ wezterm cli split-pane --bottom --cwd %val{client_env_PWD} -- %arg{@} }

define-command wezterm-new-window \
	-docstring 'Open a new wezterm window' \
	-override \
	-shell-completion \
	-params .. \
	%{ wezterm cli spawn --new-window --cwd %val{client_env_PWD} -- %arg{@} }

define-command vsplit \
	-docstring 'Open the current buffer in vertical split' \
	-override \
	%{ wezterm-vsplit kak -c %val{session} }

define-command split \
	-docstring 'Open the current buffer in horizontal split' \
	-override \
	%{ wezterm-split kak -c %val{session} }

define-command tab \
	-docstring 'Open the current buffer in horizontal split' \
	-override \
	%{ wezterm-tab kak -c %val{session} }

alias global wnw wezterm-new-window
alias global wt wezterm-tab
alias global wvs wezterm-vsplit
alias global ws wezterm-split

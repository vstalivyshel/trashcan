hook global InsertChar j %{ try %{
    exec -draft hH<a-k>kj<ret>d
    exec <esc>
} }

map global normal X 'x_'
map global normal '#' ':comment-line<ret>'
map global normal <c-g> ':pwd<ret>'

map global insert <c-u> '<esc>h<a-h>"_di'
map global insert <c-w> '<esc>"_bdi'
map global insert <c-h>   '<esc>;h"_di'

map global user ';' 'y:<c-r>"<ret>' \
	-docstring 'Push selected into cmd'
map global user ':' 'y:<c-r>"<ret>' \
	-docstring 'Push selected into cmd for all sessions'
map global user / ':split<ret>' \
	-docstring 'Open current buffer in new window'
map global user b ':find-buffer<ret>' \
	-docstring 'Serch for the active buffers'
map global user f ':find-file<ret>' \
	-docstring 'Search for the file'
map global user t ':term<ret>' \
	-docstring 'Open a terminal in new window'

map global user y '<a-|>xsel --input --clipboard<ret>'\
	-docstring 'Yank into system Clipboard'
map global user P '!xsel --output --clipboard<ret>'\
	-docstring 'Paste after a seletction from the system clipboard'
map global user p '<a-!>xsel --output --clipboard<ret>'\
	-docstring 'Paste before a seletction from the system clipboard'

map global user l ':enter-user-mode lsp<ret>' \
	-docstring 'Enter lsp-mode'
map global insert <tab> '<a-;>:try lsp-snippets-select-next-placeholders catch %{ execute-keys -with-hooks <lt>tab> }<ret>'\
	-docstring 'Select next snippet placeholder'
map global user d ':lsp-diagnostics<ret>'\
	-docstring 'Open LSP diagnostics window'
map global user k ':lsp-hover<ret>'\
	-docstring 'View LSP hover'
map global user a ':lsp-code-actions<ret>'\
	-docstring 'LPS code actions'
map global object f '<a-semicolon>lsp-object Function Method<ret>'\
	-docstring 'LSP function or method'
map global object t '<a-semicolon>lsp-object Class Interface Struct<ret>'\
	-docstring 'LSP class interface or struct'
map global object d '<a-semicolon>lsp-diagnostic-object --include-warnings<ret>'\
	-docstring 'LSP errors and warnings'

declare-user-mode window-mode
map global normal <c-w> ':enter-user-mode window-mode<ret>' \
	-docstring 'Window Mode'
map global window-mode v ':vsplit<ret>' \
	-docstring 'Open the current buffer in vertical split'
map global window-mode s ':split<ret>' \
	-docstring 'Open the current buffer in horizontal split'
map global window-mode t ':tab<ret>' \
	-docstring 'Open the current buffer in new tab'
map global window-mode q ':q<ret>' \
	-docstring 'Close current window'


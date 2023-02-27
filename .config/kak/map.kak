hook global InsertChar j %{ try %{
    exec -draft hH<a-k>kj<ret>d
    exec <esc>
} }

declare-user-mode extended
map global extended w <s-w>
map global extended b <s-b>
map global extended e <s-e>
map global extended l <s-l>
map global extended k <s-k>
map global extended j <s-j>
map global extended h <s-h>
map global extended <s-j> xdp 
map global extended <s-k> xdkP
map global extended x x
define-command -override extended-mode %{
    set-option global autoinfo ''
    enter-user-mode -lock extended
}

map global normal X 'x_'
map global normal '#' ':comment-line<ret>'

map global insert <c-u> '<esc>h<a-h>"_di'
map global insert <c-w> '<esc>"_bdi'
map global insert <c-h>   '<esc>;h"_di'  

map global user b ':find-buffer<ret>'
map global user f ':find-file<ret>'

map global user y '<a-|>xsel --input --clipboard<ret>'\
	-docstring 'Yank into system Clipboard'
map global user P '!xsel --output --clipboard<ret>'\
	-docstring 'Paste after a seletction from the system clipboard'
map global user p '<a-!>xsel --output --clipboard<ret>'\
	-docstring 'Paste before a seletction from the system clipboard'
	
map global insert <tab> '<a-;>:try lsp-snippets-select-next-placeholders catch %{ execute-keys -with-hooks <lt>tab> }<ret>'\
	-docstring 'Select next snippet placeholder'
map global user d ':lsp-diagnostics<ret>'\
	-docstring 'Open LSP diagnostics window'
map global user k ':lsp-hover<ret>'\
	-docstring 'View LSP hover'
map global user a ':lsp-code-actions<ret>'\
	-docstring 'View LSP hover'
map global object f '<a-semicolon>lsp-object Function Method<ret>'\
	-docstring 'LSP function or method'
map global object t '<a-semicolon>lsp-object Class Interface Struct<ret>'\
	-docstring 'LSP class interface or struct'
map global object d '<a-semicolon>lsp-diagnostic-object --include-warnings<ret>'\
	-docstring 'LSP errors and warnings'


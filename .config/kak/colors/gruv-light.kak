# https://github.com/helix-editor/helix/blob/master/runtime/themes/gruvbox_light.toml

declare-option -hidden str bg0             "rgb:fbf1c7"
declare-option -hidden str bg1             "rgb:ebdbb2"
declare-option -hidden str bg2             "rgb:d5c4a1"
declare-option -hidden str bg3             "rgb:bdae93"
declare-option -hidden str bg4             "rgb:a89984"
declare-option -hidden str fg0             "rgb:282828"
declare-option -hidden str fg1             "rgb:3c3836"
declare-option -hidden str fg2             "rgb:504945"
declare-option -hidden str fg3             "rgb:665c54"
declare-option -hidden str fg4             "rgb:7c6f64"
declare-option -hidden str gray0           "rgb:7c6f64"
declare-option -hidden str gray1           "rgb:928374"
declare-option -hidden str red0            "rgb:cc241d"
declare-option -hidden str red1            "rgb:9d0006"
declare-option -hidden str green0          "rgb:98971a"
declare-option -hidden str green1          "rgb:79740e"
declare-option -hidden str yellow0         "rgb:d79921"
declare-option -hidden str yellow1         "rgb:b57614"
declare-option -hidden str blue0           "rgb:458588"
declare-option -hidden str blue1           "rgb:076678"
declare-option -hidden str purple0         "rgb:b16286"
declare-option -hidden str purple1         "rgb:8f3f71"
declare-option -hidden str aqua0           "rgb:689d6a"
declare-option -hidden str aqua1           "rgb:427b58"
declare-option -hidden str orange0         "rgb:d65d0e"
declare-option -hidden str orange1         "rgb:af3a03"

# Markup
set-face	global	title				   "%opt{orange0}+b"
set-face	global	header				   "%opt{aqua1}"
set-face	global	bold				   "%opt{aqua0}"
set-face	global	italic				   "%opt{aqua0}"
set-face	global	mono				   "%opt{fg4}"
set-face	global	block				   "%opt{aqua0}"
set-face	global	link				   "%opt{green1}+c"
set-face	global	bullet				   "%opt{orange0}"
set-face	global	list				   "%opt{fg0}"

# Builtins
set-face	global	Default				   "%opt{fg1},%opt{bg0}"
set-face	global	PrimarySelection	   "default,%opt{bg2}+g"
set-face	global	SecondarySelection	   "default,%opt{bg3}+g"
set-face	global	PrimaryCursor		   "%opt{bg0},%opt{blue1}+g"
set-face	global	PrimaryCursorEol	   "%opt{bg0},%opt{blue1}+g"
set-face	global	SecondaryCursor		   "%opt{bg0},%opt{blue0}+g"
set-face	global	SecondaryCursorEol	   "%opt{bg0},%opt{blue0}+g"
set-face	global	LineNumbers			   "%opt{bg4},%opt{bg0}"
set-face	global	LineNumberCursor	   "%opt{yellow1},%opt{bg0}+b"
set-face	global	LineNumbersWrapped	   LineNumbers
set-face	global	MenuForeground 		   "%opt{bg2},%opt{blue1}+b"
set-face	global	MenuBackground		   "%opt{fg1},%opt{bg2}"
set-face	global	MenuInfo			   "%opt{aqua1},default"
set-face	global	Information			   MenuBackground
set-face	global	Error				   "%opt{red1},default"
set-face	global	StatusLine			   MenuBackground
set-face	global	StatusLineMode		   "%opt{fg1},%opt{blue0}+b"
set-face	global	StatusLineInfo		   StatusLine
set-face	global	StatusLineValue		   StatusLine
set-face	global	StatusCursor		   PrimaryCursor
set-face	global	Prompt				   "%opt{fg1},%opt{orange0}"
# set-face	global	MatchingChar		   "%opt{},%opt{}"
set-face	global	Whitespace			   Default
set-face	global	WrapMarker			   Whitespace
set-face	global	BufferPadding		   LineNumbers

# Code
set-face	global	value				   "%opt{blue1}+b"
set-face	global	type				   "%opt{yellow1}"
set-face	global	variable			   "%opt{fg1}"
set-face	global	module				   "%opt{aqua1}"
set-face	global	function			   "%opt{green1}+b"
set-face	global	string				   "%opt{green1}"
set-face	global	keyword				   "%opt{red1}"
set-face	global	operator			   "%opt{fg1}"
set-face	global	attribute			   "%opt{aqua1}"
set-face	global	comment				   "%opt{gray1}+i"
set-face	global	documentation		   comment
set-face	global	meta				   "%opt{red0}"
set-face	global	builtin				   "%opt{fg1}"

# Lsp
set-face    global  DiagnosticError        "%opt{red1}" 
set-face    global  DiagnosticWarning      "%opt{orange1}"
set-face    global  DiagnosticHint         "%opt{blue1}"
set-face    global  DiagnosticInfo         "%opt{aqua1}"
set-face    global  LineFlagError          DiagnosticError
set-face    global  LineFlagWarning        DiagnosticWarning
set-face    global  LineFlagHint           DiagnosticHint
set-face    global  LineFlagInfo           DiagnosticInfo
set-face    global  InlayDiagnosticError   DiagnosticError
set-face    global  InlayDiagnosticWarning DiagnosticWarning
set-face    global  InlayDiagnosticHint    DiagnosticHint
set-face    global  InlayDiagnosticInfo    DiagnosticInfo
set-face    global  InlayCodeLens          DiagnosticInfo
set-face    global  InlayHint              DiagnosticHint

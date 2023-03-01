# Thanks to https://github.com/helix-editor/helix/blob/master/runtime/themes/rose_pine.toml
# And to https://github.com/rose-pine/neovim

declare-option -hidden str base            "rgb:faf4ed" 
declare-option -hidden str surface         "rgb:fffaf3" 
declare-option -hidden str overlay         "rgb:f2e9e1"
declare-option -hidden str muted           "rgb:9893a5"
declare-option -hidden str subtle          "rgb:797593"
declare-option -hidden str text            "rgb:575279"
declare-option -hidden str love            "rgb:b4637a"
declare-option -hidden str love_10         "rgb:f6e4e0"
declare-option -hidden str gold            "rgb:ea9d34"
declare-option -hidden str gold_10         "rgb:fbead8"
declare-option -hidden str rose            "rgb:d7827e"
declare-option -hidden str rose_10         "rgb:fae8e1"
declare-option -hidden str pine            "rgb:286983"
declare-option -hidden str pine_10         "rgb:e5e6e2"
declare-option -hidden str foam            "rgb:56949f"
declare-option -hidden str foam_10         "rgb:eaeae5"
declare-option -hidden str iris            "rgb:907aa9"
declare-option -hidden str iris_10         "rgb:f1e8e6"
declare-option -hidden str highlight_low   "rgb:f4ede8"
declare-option -hidden str highlight_med   "rgb:dfdad9"
declare-option -hidden str highlight_high  "rgb:cecacd"

# Markup
set-face	global	title				   "%opt{rose}+b"
set-face	global	header				   "%opt{rose}"
set-face	global	bold				   "%opt{rose}"
set-face	global	italic				   "%opt{rose}"
set-face	global	mono				   "%opt{rose}"
set-face	global	block				   "%opt{pine}"
set-face	global	link				   "%opt{love}+c"
set-face	global	bullet				   "%opt{rose}"
set-face	global	list				   "%opt{rose}"

# Builtins
set-face	global	Default				   "%opt{text},%opt{base}"
set-face	global	PrimarySelection	   "default,%opt{highlight_med}+g"
set-face	global	SecondarySelection	   "default,%opt{highlight_med}+g"
set-face	global	PrimaryCursor		   "%opt{text},%opt{muted}+g"
set-face	global	SecondaryCursor		   "%opt{text},%opt{highlight_high}+g"
set-face	global	PrimaryCursorEol	   "%opt{text},%opt{muted}"
set-face	global	SecondaryCursorEol	   "%opt{text},%opt{highlight_high}+g"
set-face	global	LineNumbers			   "%opt{muted},%opt{base}"
set-face	global	LineNumberCursor	   "%opt{text},%opt{base}+b"
set-face	global	LineNumbersWrapped	   LineNumbers
set-face	global	MenuForeground 		   "%opt{text},%opt{surface}+b"
set-face	global	MenuBackground		   "%opt{subtle},%opt{surface}"
set-face	global	MenuInfo			   "%opt{rose},%opt{surface}"
set-face	global	Information			   MenuBackground
set-face	global	Error				   "%opt{surface},%opt{love}"
set-face	global	StatusLine			   MenuBackground
set-face	global	StatusLineMode		   "%opt{surface},%opt{rose}+b"
set-face	global	StatusLineInfo		   StatusLine
set-face	global	StatusLineValue		   StatusLine
set-face	global	StatusCursor		   PrimaryCursor
set-face	global	Prompt				   "%opt{surface},%opt{foam}"
set-face	global	MatchingChar		   "%opt{rose},%opt{base}"
set-face	global	Whitespace			   "%opt{text},%opt{base}"
set-face	global	WrapMarker			   Whitespace
set-face	global	BufferPadding		   LineNumbers

# Code
set-face	global	value				   "%opt{iris}"
set-face	global	type				   "%opt{foam}"
set-face	global	variable			   "%opt{text}"
set-face	global	module				   "%opt{iris}"
set-face	global	function			   "%opt{rose}"
set-face	global	string				   "%opt{gold}"
set-face	global	keyword				   "%opt{pine}"
set-face	global	operator			   "%opt{text}"
set-face	global	attribute			   "%opt{iris}"
set-face	global	comment				   "%opt{muted}+i"
set-face	global	documentation		   comment
set-face	global	meta				   "%opt{rose}"
set-face	global	builtin				   "%opt{rose}+b"

# Lsp
set-face    global  DiagnosticError        "%opt{love}" 
set-face    global  DiagnosticWarning      "%opt{gold}"
set-face    global  DiagnosticHint         "%opt{iris}"
set-face    global  DiagnosticInfo         "%opt{foam}"
set-face    global  LineFlagError          "%opt{love}" 
set-face    global  LineFlagWarning        "%opt{gold}"
set-face    global  LineFlagHint           "%opt{iris}"
set-face    global  LineFlagInfo           "%opt{foam}"
set-face    global  InlayDiagnosticError   "%opt{love}" 
set-face    global  InlayDiagnosticWarning "%opt{gold}"
set-face    global  InlayDiagnosticHint    "%opt{iris}"
set-face    global  InlayDiagnosticInfo    "%opt{foam}"
set-face    global  InlayCodeLens          "%opt{foam}"
set-face    global  InlayHint              "%opt{iris}"


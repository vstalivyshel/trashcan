# Thanks to https://github.com/helix-editor/helix/blob/master/runtime/themes/rose_pine.toml
# And to https://github.com/rose-pine/neovim

declare-option -hidden str base                 "rgb:191724" 
declare-option -hidden str surface              "rgb:1f1d2e" 
declare-option -hidden str overlay              "rgb:26233a"
declare-option -hidden str muted                "rgb:6e6a86"
declare-option -hidden str subtle               "rgb:908caa"
declare-option -hidden str text                 "rgb:e0def4"
declare-option -hidden str love                 "rgb:eb6f92"
declare-option -hidden str love_10              "rgb:311f30"
declare-option -hidden str gold                 "rgb:f6c177"
declare-option -hidden str gold_10              "rgb:30282c"
declare-option -hidden str rose                 "rgb:ebbcba"
declare-option -hidden str rose_10              "rgb:2f2834"
declare-option -hidden str pine                 "rgb:31748f"
declare-option -hidden str pine_10              "rgb:1a2030"
declare-option -hidden str foam                 "rgb:9ccfd8"
declare-option -hidden str foam_10              "rgb:252937"
declare-option -hidden str iris                 "rgb:c4a7e7"
declare-option -hidden str iris_10              "rgb:2b2539"
declare-option -hidden str highlight_low        "rgb:21202e"
declare-option -hidden str highlight_med        "rgb:403d52"
declare-option -hidden str highlight_high       "rgb:524f67"

# Markup
set-face	global	title				"%opt{rose}+b"
set-face	global	header				"%opt{rose}"
set-face	global	bold				"%opt{rose}"
set-face	global	italic				"%opt{rose}"
set-face	global	mono				"%opt{rose}"
set-face	global	block				"%opt{pine}"
set-face	global	link				"%opt{love}+c"
set-face	global	bullet				"%opt{rose}"
set-face	global	list				"%opt{rose}"

# Builtins
set-face	global	Default				"%opt{text},%opt{base}"
set-face	global	PrimarySelection	"default,%opt{overlay}+g"
set-face	global	SecondarySelection	"default,%opt{highlight_med}+g"
set-face	global	PrimaryCursor		"%opt{text},%opt{muted}+g"
set-face	global	SecondaryCursor		"%opt{text},%opt{highlight_high}+g"
set-face	global	PrimaryCursorEol	"%opt{text},%opt{muted}"
set-face	global	SecondaryCursorEol	"%opt{text},%opt{highlight_high}+g"
set-face	global	LineNumbers			"%opt{muted},%opt{base}"
set-face	global	LineNumberCursor	"%opt{text},%opt{base}+b"
set-face	global	LineNumbersWrapped	LineNumbers
set-face	global	MenuForeground 		"%opt{text},%opt{surface}+b"
set-face	global	MenuBackground		"%opt{subtle},%opt{surface}"
set-face	global	MenuInfo			"%opt{rose},%opt{surface}"
set-face	global	Information			MenuBackground
set-face	global	Error				"%opt{surface},%opt{love}"
set-face	global	StatusLine			MenuBackground
set-face	global	StatusLineMode		"%opt{surface},%opt{rose}+b"
set-face	global	StatusLineInfo		StatusLine
set-face	global	StatusLineValue		StatusLine
set-face	global	StatusCursor		PrimaryCursor
set-face	global	Prompt				"%opt{surface},%opt{foam}"
set-face	global	MatchingChar		"%opt{rose},%opt{base}"
set-face	global	Whitespace			"%opt{text},%opt{base}"
set-face	global	WrapMarker			Whitespace
set-face	global	BufferPadding		LineNumbers

# Code
set-face	global	value				"%opt{iris}"
set-face	global	type				"%opt{foam}"
set-face	global	variable			"%opt{text}"
set-face	global	module				"%opt{iris}"
set-face	global	function			"%opt{rose}"
set-face	global	string				"%opt{gold}"
set-face	global	keyword				"%opt{pine}"
set-face	global	operator			"%opt{text}"
set-face	global	attribute			"%opt{iris}"
set-face	global	comment				"%opt{muted}+i"
set-face	global	documentation		comment
set-face	global	meta				"%opt{rose}"
set-face	global	builtin				"%opt{rose}+b"

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


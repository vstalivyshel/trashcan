# Thanks to https://github.com/helix-editor/helix/blob/master/runtime/themes/kanagawa.toml
# And to https://github.com/catppuccin/kakoune
# And to https://github.com/rebelot/kanagawa.nvim

declare-option str seaFoam                 "rgb:C7CCD1" # custom lighter foreground
declare-option str fujiWhite               "rgb:DCD7BA" # default foreground
declare-option str oldWhite                "rgb:C8C093" # dark foreground, e.g. statuslines
declare-option str sumiInk0                "rgb:16161D" # dark background, e.g. statuslines, floating windows
declare-option str sumiInk1                "rgb:1F1F28" # default background
declare-option str sumiInk2                "rgb:2A2A37" # lighter background, e.g. colorcolumns, folds
declare-option str sumiInk3                "rgb:363646" # lighter background, e.g. cursorline
declare-option str sumiInk4                "rgb:54546D" # darker foreground, e.g. linenumbers, fold column
declare-option str waveBlue1               "rgb:223249" # popup background, visual selection background
declare-option str waveBlue2               "rgb:2D4F67" # popup selection background, search background
declare-option str winterGreen             "rgb:2B3328" # diff add background
declare-option str winterYellow            "rgb:49443C" # diff change background
declare-option str winterRed               "rgb:43242B" # diff deleted background
declare-option str winterBlue              "rgb:252535" # diff line background
declare-option str autumnGreen             "rgb:76946A" # git add
declare-option str autumnRed               "rgb:C34043" # git delete
declare-option str autumnYellow            "rgb:DCA561" # git change
declare-option str samuraiRed              "rgb:E82424" # diagnostic error
declare-option str roninYellow             "rgb:FF9E3B" # diagnostic warning
declare-option str waveAqua1               "rgb:6A9589" # diagnostic info
declare-option str dragonBlue              "rgb:658594" # diagnostic hint
declare-option str fujiGray                "rgb:727169" # comments
declare-option str springViolet1           "rgb:938AA9" # light foreground
declare-option str oniViolet               "rgb:957FB8" # statements and keywords
declare-option str crystalBlue             "rgb:7E9CD8" # functions and titles
declare-option str springViolet2           "rgb:9CABCA" # brackets and punctuation
declare-option str springBlue              "rgb:7FB4CA" # specials and builtins
declare-option str lightBlue               "rgb:A3D4D5" # not used!
declare-option str waveAqua2               "rgb:7AA89F" # types
declare-option str springGreen             "rgb:98BB6C" # strings
declare-option str boatYellow1             "rgb:938056" # not used
declare-option str boatYellow2             "rgb:C0A36E" # operators, regex
declare-option str carpYellow              "rgb:E6C384" # identifiers
declare-option str sakuraPink              "rgb:D27E99" # numbers
declare-option str waveRed                 "rgb:E46876" # standout specials 1, e.g. builtin variables
declare-option str peachRed                "rgb:FF5D62" # standout specials 2, e.g. exception handling, returns
declare-option str surimiOrange            "rgb:FFA066" # constants, imports, booleans
declare-option str katanaGray              "rgb:717C7C" # deprecated

# Markup
set-face	global	title				   "%opt{springViolet2}"
set-face	global	header				   "%opt{surimiOrange}+b"
set-face	global	bold				   "%opt{fujiWhite}+b"
set-face	global	italic				   "%opt{fujiWhite}+i"
set-face	global	mono				   "%opt{fujiWhite}"
set-face	global	block				   "%opt{fujiWhite}"
set-face	global	link				   "%opt{springBlue}+c"
set-face	global	bullet				   "%opt{springViolet1}"
set-face	global	list				   "%opt{sakuraPink}"

# Builtins
set-face	global	Default				   "%opt{fujiWhite},%opt{sumiInk1}"
set-face	global	PrimarySelection	   "default,%opt{sumiInk3}+g"
set-face	global	SecondarySelection	   "default,%opt{sumiInk2}+g"
set-face	global	PrimaryCursor		   "%opt{sumiInk1},%opt{fujiWhite}+g"
set-face	global	SecondaryCursor		   "%opt{sumiInk1},%opt{waveAqua1}+g"
set-face	global	PrimaryCursorEol	   "%opt{sumiInk1},%opt{fujiWhite}+g"
set-face	global	SecondaryCursorEol	   "%opt{sumiInk1},%opt{waveAqua2}+g"
set-face	global	LineNumbers			   "%opt{sumiInk4},%opt{sumiInk1}"
set-face	global	LineNumberCursor	   "%opt{roninYellow},%opt{sumiInk1}+b"
set-face	global	LineNumbersWrapped	   "%opt{sumiInk4},%opt{sumiInk1}"
set-face	global	MenuForeground 		   "%opt{fujiWhite},%opt{sumiInk2}+b"
set-face	global	MenuBackground		   "%opt{fujiWhite},%opt{sumiInk0}"
set-face	global	MenuInfo			   "%opt{crystalBlue},%opt{sumiInk0}"
set-face	global	Information			   "%opt{fujiWhite},%opt{sumiInk0}"
set-face	global	Error				   "%opt{sumiInk0},%opt{waveRed}"
set-face	global	StatusLine			   "%opt{oldWhite},%opt{sumiInk0}"
set-face	global	StatusLineMode		   "%opt{sumiInk0},%opt{crystalBlue}"
set-face	global	StatusLineInfo		   "%opt{oldWhite},%opt{sumiInk0}"
set-face	global	StatusLineValue		   "%opt{sumiInk0},%opt{autumnGreen}"
set-face	global	StatusCursor		   "%opt{sumiInk0},%opt{fujiWhite}"
set-face	global	Prompt				   "%opt{sumiInk0},%opt{autumnGreen}"
set-face	global	MatchingChar		   "%opt{waveRed},%opt{sumiInk1}+b"
set-face	global	Whitespace			   "%opt{fujiWhite},%opt{sumiInk1}+f"
set-face	global	WrapMarker			   Whitespace
set-face	global	BufferPadding		   "%opt{sumiInk4},%opt{sumiInk1}"

# Code
set-face	global	value				   "%opt{carpYellow}"
set-face	global	type				   "%opt{waveAqua2}"
set-face	global	variable			   "%opt{fujiWhite}"
set-face	global	module				   "%opt{surimiOrange}"
set-face	global	function			   "%opt{crystalBlue}"
set-face	global	string				   "%opt{springGreen}"
set-face	global	keyword				   "%opt{oniViolet}"
set-face	global	operator			   "%opt{springViolet2}"
set-face	global	attribute			   "%opt{waveRed}"
set-face	global	comment				   "%opt{fujiGray}"
set-face	global	documentation		   comment
set-face	global	meta				   "%opt{peachRed}"
set-face	global	builtin				   "%opt{waveRed}+b"

# Lsp
set-face    global  DiagnosticError        "%opt{samuraiRed}" 
set-face    global  DiagnosticWarning      "%opt{roninYellow}"
set-face    global  DiagnosticHint         "%opt{dragonBlue}"
set-face    global  DiagnosticInfo         "%opt{waveAqua1}"
set-face    global  LineFlagError          "%opt{samuraiRed}" 
set-face    global  LineFlagWarning        "%opt{roninYellow}"
set-face    global  LineFlagHint           "%opt{dragonBlue}"
set-face    global  LineFlagInfo           "%opt{waveAqua1}"
set-face    global  InlayDiagnosticError   "%opt{samuraiRed}" 
set-face    global  InlayDiagnosticWarning "%opt{roninYellow}"
set-face    global  InlayDiagnosticHint    "%opt{dragonBlue}"
set-face    global  InlayDiagnosticInfo    "%opt{waveAqua1}"
set-face    global  InlayCodeLens          "%opt{waveAqua1}"
set-face    global  InlayHint              "%opt{waveAqua1}"



-- { <fg>, [bg], [attr] } or just <fg>
-- Color: "#RRGGBB" or 'MenuInfo = "Default"', exception: func = "function"
-- Attribute: "+..."
--- Palette ---
local default          = "default"
local base             = "#232136"
local surface          = "#2a273f"
local overlay          = "#393552"
local muted            = "#6e6a86"
local subtle           = "#908caa"
local text             = "#e0def4"
local love             = "#eb6f92"
local love_10          = "#3a2841"
local gold             = "#f6c177"
local gold_10          = "#39313e"
local rose             = "#ea9a97"
local rose_10          = "#392d41"
local pine             = "#3e8fb0"
local pine_10          = "#252c44"
local foam             = "#9ccfd8"
local foam_10          = "#2e3248"
local iris             = "#c4a7e7"
local iris_10          = "#342e4a"
local highlight_low    = "#2a283e"
local highlight_med    = "#44415a"
local highlight_high   = "#56526e"

return {
    Colorscheme_name       = "rose-pine-moon",

    --- Builtins ---
    Default                = {text,base},
    InsertCursor           = {base, rose, "+b"},
    PrimarySelection       = {default,overlay, "+g"},
    SecondarySelection     = {default, highlight_med, "+g"},
    PrimaryCursor          = {text,muted, "+g"},
    SecondaryCursor        = {text,highlight_high, "+g"},
    PrimaryCursorEol       = {text,muted},
    SecondaryCursorEol     = {text,highlight_high, "+g"},
    LineNumbers            = {muted,base},
    LineNumberCursor       = {text,base, "+b"},
    LineNumbersWrapped     = "LineNumbers",
    MenuForeground         = {text,surface, "+b"},
    MenuBackground         = {subtle,surface},
    MenuInfo               = {rose,surface},
    Information            = "MenuBackground",
    Error                  = {surface,love},
    StatusLine             = {subtle, base},
    StatusLineMode         = {surface,rose, "+b"},
    StatusLineInfo         = "StatusLine",
    StatusLineValue        = "StatusLine",
    StatusCursor           = "PrimaryCursor",
    Prompt                 = {surface,foam},
    MatchingChar           = {text,highlight_med},
    Whitespace             = {text,base},
    WrapMarker             = "Whitespace",
    BufferPadding          = "LineNumbers",

    --- Code ---
    value                  = {iris, "+b"},
    type                   = foam,
    variable               = text,
    module                 = iris,
    func                   = rose,
    string                 = gold,
    keyword                = pine,
    operator               = text,
    attribute              = iris,
    comment                = {muted, "+i"},
    documentation          = comment,
    meta                   = rose,
    builtin                = {rose, "+b"},

    --- LSP ---
    DiagnosticError        = love,
    DiagnosticWarning      = gold,
    DiagnosticHint         = iris,
    DiagnosticInfo         = foam,
    LineFlagError          = love,
    LineFlagWarning        = gold,
    LineFlagHint           = iris,
    LineFlagInfo           = foam,
    InlayDiagnosticError   = love,
    InlayDiagnosticWarning = gold,
    InlayDiagnosticHint    = iris,
    InlayDiagnosticInfo    = foam,
    InlayCodeLens          = foam,
    InlayHint              = iris,

    --- Markup ---
    title                  = {rose, "+b"},
    header                 = rose,
    bold                   = rose,
    italic                 = rose,
    mono                   = rose,
    block                  = pine,
    link                   = {love, "+c"},
    bullet                 = rose,
    list                   = rose,
}




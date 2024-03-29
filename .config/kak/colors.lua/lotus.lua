-- thx to https://github.com/rebelot/kanagawa.nvim

local default = "default"
local palette = {
    lotusInk1    = "#545464",
    lotusInk2    = "#43436c",
    lotusGray    = "#dcd7ba",
    lotusGray2   = "#716e61",
    lotusGray3   = "#8a8980",
    lotusWhite0  = "#d5cea3",
    lotusWhite1  = "#dcd5ac",
    lotusWhite2  = "#e5ddb0",
    lotusWhite3  = "#f2ecbc",
    lotusWhite4  = "#e7dba0",
    lotusWhite5  = "#e4d794",
    lotusViolet1 = "#a09cac",
    lotusViolet2 = "#766b90",
    lotusViolet3 = "#c9cbd1",
    lotusViolet4 = "#624c83",
    lotusBlue1   = "#c7d7e0",
    lotusBlue2   = "#b5cbd2",
    lotusBlue3   = "#9fb5c9",
    lotusBlue4   = "#4d699b",
    lotusBlue5   = "#5d57a3",
    lotusGreen   = "#6f894e",
    lotusGreen2  = "#6e915f",
    lotusGreen3  = "#b7d0ae",
    lotusPink    = "#b35b79",
    lotusOrange  = "#cc6d00",
    lotusOrange2 = "#e98a00",
    lotusYellow  = "#77713f",
    lotusYellow2 = "#836f4a",
    lotusYellow3 = "#de9800",
    lotusYellow4 = "#f9d791",
    lotusRed     = "#c84053",
    lotusRed2    = "#d7474b",
    lotusRed3    = "#e82424",
    lotusRed4    = "#d9a594",
    lotusAqua    = "#597b75",
    lotusAqua2   = "#5e857a",
    lotusTeal1   = "#4e8ca2",
    lotusTeal2   = "#6693bf",
    lotusTeal3   = "#5a7785",
    lotusCyan    = "#d7e3d8",
}

local lotus = {
    fg             = palette.lotusInk1,
    fg_dim         = palette.lotusInk2,
    fg_reverse     = palette.lotusGray,

    bg_dim         = palette.lotusWhite1,
    bg_gutter      = palette.lotusWhite4,

    bg_m3          = palette.lotusWhite0,
    bg_m2          = palette.lotusWhite1,
    bg_m1          = palette.lotusWhite2,
    bg             = palette.lotusWhite3,
    bg_p1          = palette.lotusWhite4,
    bg_p2          = palette.lotusWhite5,

    nontext        = palette.lotusViolet1,
    whitespace     = palette.lotusViolet1,
    special        = palette.lotusViolet2,

    bg_visual      = palette.lotusViolet3,
    bg_search      = palette.lotusBlue2,

    pmenu = {
        fg         = palette.lotusInk2,
        fg_sel     = default,
        bg         = palette.lotusBlue1,
        bg_sel     = palette.lotusBlue3,
        bg_sbar    = palette.lotusBlue1,
        bg_thumb   = palette.lotusBlue2,
    },
    float = {
        fg         = palette.lotusInk2,
        bg         = palette.lotusWhite0,
        fg_border  = palette.lotusGray2,
        bg_border  = palette.lotusWhite0,
    },
     syn = {
        string     = palette.lotusGreen,
        variable   = palette.lotusInk2,
        number     = palette.lotusPink,
        constant   = palette.lotusOrange,
        identifier = palette.lotusYellow,
        parameter  = palette.lotusBlue5,
        fun        = palette.lotusBlue4,
        statement  = palette.lotusViolet4,
        keyword    = palette.lotusViolet4,
        operator   = palette.lotusYellow2,
        preproc    = palette.lotusRed,
        type       = palette.lotusAqua,
        regex      = palette.lotusYellow2,
        deprecated = palette.lotusGray3,
        comment    = palette.lotusGray3,
        punct      = palette.lotusTeal1,
        special1   = palette.lotusTeal2,
        special2   = palette.lotusRed,
        special3   = palette.lotusRed,
    },
    diag = {
        error      = palette.lotusRed3,
        ok         = palette.lotusGreen,
        warning    = palette.lotusOrange2,
        info       = palette.lotusTeal3,
        hint       = palette.lotusAqua2,
    },
}

return {
    Colorscheme_name       = "lotus",

    --- Builtins ---
    Default                = {lotus.fg, lotus.bg},
    PrimarySelection       = {default, lotus.bg_visual, "+g"},
    SecondarySelection     = {default, palette.lotusWhite0, "+g"},
    PrimaryCursor          = {lotus.bg, lotus.fg, "+g"},
    InsertCursor           = {lotus.bg, palette.lotusGreen2},
    SecondaryCursor        = {lotus.bg, palette.lotusGreen2, "+g"},
    PrimaryCursorEol       = "PrimaryCursor",
    SecondaryCursorEol     = "SecondaryCursor",
    LineNumbers            = {lotus.nontext, lotus.bg},
    LineNumberCursor       = {lotus.nontext, lotus.bg, "+b"},
    LineNumbersWrapped     = "LineNumbers",
    MenuForeground         = {lotus.pmenu.fg_sel, lotus.pmenu.bg_sel, "+b"},
    MenuBackground         = {lotus.pmenu.fg, lotus.pmenu.bg},
    MenuInfo               = {lotus.pmenu.fg},
    Information            = "MenuBackground",
    Error                  = {lotus.bg, lotus.diag.error},
    StatusLine             = {lotus.fg, lotus.bg},
    StatusLineMode         = {lotus.bg, lotus.diag.hint, "+b"},
    StatusLineInfo         = "StatusLine",
    StatusLineValue        = "StatusLine",
    StatusCursor           = "PrimaryCursor",
    Prompt                 = {lotus.bg, lotus.diag.info},
    MatchingChar           = {lotus.diag.warning, "+b"},
    Whitespace             = {lotus.whitespace, "+f"},
    WrapMarker             = "Whitespace",
    BufferPadding          = lotus.bg,

    --- Code ---
    value                  = lotus.syn.number,
    type                   = lotus.syn.type,
    variable               = palette.lotusInk1,
    module                 = lotus.syn.preproc,
    func                   = lotus.syn.fun,
    string                 = lotus.syn.string,
    keyword                = {lotus.syn.keyword, "+i"},
    operator               = "variable",
    attribute              = palette.lotusBlue5,
    comment                = {lotus.syn.comment, "+i"},
    documentation          = "comment",
    meta                   = lotus.syn.special1,
    builtin                = {lotus.syn.statement, "+b"},

    --- LSP ---
    DiagnosticError        = lotus.diag.error,
    DiagnosticWarning      = lotus.diag.warning,
    DiagnosticHint         = lotus.diag.hint,
    DiagnosticInfo         = lotus.diag.info,
    LineFlagError          = "DiagnosticError",
    LineFlagWarning        = "DiagnosticWarning",
    LineFlagHint           = "DiagnosticHint",
    LineFlagInfo           = "DiagnosticInfo",
    InlayDiagnosticError   = "DiagnosticError",
    InlayDiagnosticWarning = "DiagnosticWarning",
    InlayDiagnosticHint    = "DiagnosticHint",
    InlayDiagnosticInfo    = "DiagnosticInfo",
    InlayCodeLens          = "DiagnosticInfo",
    InlayHint              = "DiagnosticInfo",

    --- Markup ---
    title                  = lotus.syn.keyword,
    -- header                 = {surimiOrange, "+b"},
    -- bold                   = {fujiWhite, "+b"},
    -- italic                 = {fujiWhite, "+i"},
    -- mono                   = fujiWhite,
    -- block                  = fujiWhite,
    -- link                   = {springBlue, "+c"},
    -- bullet                 = springViolet1,
    -- list                   = sakuraPink,
}


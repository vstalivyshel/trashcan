#!/bin/env python

default = "default"
g = "g"
b = "b"
f = "f"
i = "i"
c = "c"
u = "u"

colorscheme_name = "gruv-light"

bg0 = "#fbf1c7"
bg1 = "#ebdbb2"
bg2 = "#d5c4a1"
bg3 = "#bdae93"
bg4 = "#a89984"
fg0 = "#282828"
fg1 = "#3c3836"
fg2 = "#504945"
fg3 = "#665c54"
fg4 = "#7c6f64"
gray0 = "#7c6f64"
gray1 = "#928374"
red0 = "#cc241d"
red1 = "#9d0006"
green0 = "#98971a"
green1 = "#79740e"
yellow0 = "#d79921"
yellow1 = "#b57614"
blue0 = "#458588"
blue1 = "#076678"
purple0 = "#b16286"
purple1 = "#8f3f71"
aqua0 = "#689d6a"
aqua1 = "#427b58"
orange0 = "#d65d0e"
orange1 = "#af3a03"

gruvbox_light = {
    # Builtins
    "Default": [fg1, bg0],
    "PrimarySelection": [default, bg2, g],
    "SecondarySelection": [default, bg3, g],
    "PrimaryCursor": [bg0, blue1, g],
    "PrimaryCursorEol": [bg0, blue1, g],
    "SecondaryCursor": [bg0, blue0, g],
    "SecondaryCursorEol": [bg0, blue0, g],
    "LineNumbers": [bg4, bg0],
    "LineNumberCursor": [yellow1, bg0, b],
    "LineNumbersWrapped": "LineNumbers",
    "MenuForeground": [bg2, blue1, b],
    "MenuBackground": [fg1, bg2],
    "MenuInfo": [aqua1, default],
    "Information": "MenuBackground",
    "Error": [red1, default],
    "StatusLine": "MenuBackground",
    "StatusLineMode": [fg1, blue0, b],
    "StatusLineInfo": "StatusLine",
    "StatusLineValue": "StatusLine",
    "StatusCursor": "PrimaryCursor",
    "Prompt": [fg1, orange0],
    # "MatchingChar": [],
    "Whitespace": "Default",
    "WrapMarker": "Whitespace",
    "BufferPadding": "LineNumbers",

    # Code
    "value": [blue1, b],
    "type": [yellow1],
    "variable": [fg1],
    "module": [aqua1],
    "function": [green1, b],
    "string": [green1],
    "keyword": [red1],
    "operator": [fg1],
    "attribute": [aqua1],
    "comment": [gray1, i],
    "documentation": "comment",
    "meta": [red0],
    "builtin": [fg1],

    # LSP
    "DiagnosticError": [red1],
    "DiagnosticWarning": [orange1],
    "DiagnosticHint": [blue1],
    "DiagnosticInfo": [aqua1],
    "LineFlagError": "DiagnosticError",
    "LineFlagWarning": "DiagnosticWarning",
    "LineFlagHint": "DiagnosticHint",
    "LineFlagInfo": "DiagnosticInfo",
    "InlayDiagnosticError": "DiagnosticError",
    "InlayDiagnosticWarning": "DiagnosticWarning",
    "InlayDiagnosticHint": "DiagnosticHint",
    "InlayDiagnosticInfo": "DiagnosticInfo",
    "InlayCodeLens": "DiagnosticInfo",
    "InlayHint": "DiagnosticHint",

    # Markup
    "title": [orange0, b],
    "header": [aqua1],
    "bold": [aqua0],
    "italic": [aqua0],
    "mono": [fg4],
    "block": [aqua0],
    "link": [green1, c],
    "bullet": [orange0],
    "list": [fg0],
}

colorscheme_path = f"/home/vstalivyshel/.config/kak/colors/{colorscheme_name}.kak"
open(colorscheme_path, "w").close()


def main():
    for name, color in gruvbox_light.items():
        if len(color) == 3:
            fg = color[0].replace("#", "rgb:")
            bg = color[1].replace("#", "rgb:")
            opt = color[2]
            color = f"{fg},{bg}+{opt}"
        elif len(color) == 2:
            fg = color[0].replace("#", "rgb:")
            bg_or_opt = color[1].replace("#", "rgb:")
            if len(bg_or_opt) != 1 or bg_or_opt == "default":
                color = f"{fg},{bg_or_opt}"
            else:
                color = f"{fg}+{bg_or_opt}"
        elif isinstance(color, str):
            color = color
        else:
            fg = color[0].replace("#", "rgb:")
            color = fg

        echo_face = f'face global {name} "{color}"\n'
        with open(colorscheme_path, "a") as f:
            f.write(echo_face)


if __name__ == "__main__":
    main()

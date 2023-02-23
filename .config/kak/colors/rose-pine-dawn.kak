evaluate-commands %sh{
    
base=rgb:faf4ed
surface=rgb:fffaf3
overlay=rgb:f2e9de
inactive=rgb:9893a5
subtle=rgb:6e6a86
text=rgb:575279
love=rgb:b4637a
gold=rgb:ea9d34
rose=rgb:d7827e
pine=rgb:286983
foam=rgb:56949f
iris=rgb:907aa9
highlight=rgb:eee9e6
highlightInactive=rgb:f2ede9
highlightOverlay=rgb:e4dfde

    
## code
echo "
    face global value ${iris}
    face global type ${iris}
    face global identifier ${subtle}
    face global string ${gold}
    face global keyword ${pine}
    face global operator default
    face global attribute ${foam}
    face global comment ${subtle}+i
    face global meta ${foam}
    face global builtin ${pine}

    face global variable ${rose}
    face global module ${foam}
    face global function ${foam}
"

## markup
echo "
    face global title ${rose}+b
    face global header ${rose}
    face global bold default
    face global italic default
    face global mono ${rose}
    face global block ${pine}
    face global link ${love}
    face global bullet ${rose}
    face global list ${rose}
"

## builtin
echo "
    face global Default ${text},${base}
    face global PrimarySelection default,${highlight}
    face global SecondarySelection default,${highlightInactive}
    face global PrimaryCursor ${base},${text}
    face global SecondaryCursor default,${inactive}
    face global LineNumbers ${subtle},${base}
    face global LineNumberCursor ${gold},${base}
    face global MenuForeground ${text},${overlay}
    face global MenuBackground ${subtle},${surface}
    face global MenuInfo ${inactive}
    face global Information ${text},${overlay}
    face global Error ${base},${love}
    face global StatusLine ${text},${surface}
    face global StatusLineMode ${rose}
    face global StatusLineInfo ${foam}
    face global StatusLineValue ${pine}
    face global StatusCursor ${base},${text}
    face global Prompt ${foam},${surface}
    face global MatchingChar default+u
    face global BufferPadding ${inactive},${base}
    face global Whitespace ${inactive}+f
"

}

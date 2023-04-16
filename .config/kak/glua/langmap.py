#!/bin/env python

import sys

DEFAULT = "en"
SCOPE = "buffer"
MODE = "insert"
LANGMAP = {
    "en": "`~1!2@3#4$5%6^7&8*9(0)_=+\\|qQwWeErRtTyYuUiIoOpP[{]}aAsSdDfFgGhHjJkKlL;:'\"zZxXcCvVbBnNmM,<.>/?",
    "ru": 'ёЁ1!2"3№4;5%6:7?8*9(0)_=+\\/йЙцЦуУкКеЕнНгГшШщЩзЗхХъЪфФыЫвВаАпПрРоОлЛдДжЖэЭяЯчЧсСмМиИтТьЬбБюЮ.,',
    "ua": "'ʼ1!2\"3№4;5%6:7?8*9(0)_=+ґҐйЙцЦуУкКеЕнНгГшШщЩзЗхХїЇфФіІвВаАпПрРоОлЛдДжЖєЄяЯчЧсСмМиИтТьЬбБюЮ.,",
}


def main():
    if len(sys.argv) < 2:
        return

    current_lang = sys.argv[1]
    alt_lang = sys.argv[2]

    if alt_lang == DEFAULT:
        return
    elif alt_lang not in LANGMAP.keys():
        return
    elif len(alt_lang) != len(DEFAULT):
        return

    action = "map"
    if current_lang == alt_lang:
        action = "unmap"

    default = list(LANGMAP.get(DEFAULT))
    alt = list(LANGMAP.get(alt_lang))
    for i in range(len(default)):
        print(f"{action} {SCOPE} {MODE} %§{default[i]}§ %§{alt[i]}§")


if __name__ == "__main__":
    main()

#!/bin/env python

import sys, os

LANGMAP = {
    "en": "`~1!2@3#4$5%6^7&8*9(0)_=+\\|qQwWeErRtTyYuUiIoOpP[{]}aAsSdDfFgGhHjJkKlL;:'\"zZxXcCvVbBnNmM,<.>/?",
    "ru": 'ёЁ1!2"3№4;5%6:7?8*9(0)_=+\\/йЙцЦуУкКеЕнНгГшШщЩзЗхХъЪфФыЫвВаАпПрРоОлЛдДжЖэЭяЯчЧсСмМиИтТьЬбБюЮ.,',
    "ua": "'ʼ1!2\"3№4;5%6:7?8*9(0)_=+ґҐйЙцЦуУкКеЕнНгГшШщЩзЗхХїЇфФіІвВаАпПрРоОлЛдДжЖєЄяЯчЧсСмМиИтТьЬбБюЮ.,",
}

DEFAULT = "en"
SCOPE = "buffer"
MODE = "insert"


def langmap_extention():
    if len(sys.argv) < 2:
        print(
            "fail Wrong argument count! Usage: <current langmap> <alt langmap> <client>"
        )
        return

    current_lang = sys.argv[1]
    alt_lang = sys.argv[2]

    if alt_lang == DEFAULT:
        print(
            f"fail Invalid alternative langmap: {alt_lang}! Set not the default one! Default: {DEFAULT}!"
        )
        return
    elif alt_lang not in LANGMAP.keys():
        print(
            f"fail Set different alternative langmap! This one is not in the list: {alt_lang}"
        )
        return
    elif len(alt_lang) != len(DEFAULT):
        print(
            f"fail wrong char length: {DEFAULT} = {len(DEFAULT)} , {alt_lang} = {len(alt_lang)}"
        )
        return

    action = "map"
    if current_lang == alt_lang:
        action = "unmap"

    default = list(LANGMAP.get(DEFAULT))
    alt = list(LANGMAP.get(alt_lang))
    for i in range(len(default)):
        print(f"{action} {SCOPE} {MODE} %🦀{default[i]}🦀 %🦀{alt[i]}🦀")


if __name__ == "__main__":
    langmap_extention()

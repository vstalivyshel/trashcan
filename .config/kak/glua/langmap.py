#!/bin/env python

import sys

DEFAULT = "en"
LANGMAP = {
    "en": "`~1!2@3#4$5%6^7&8*9(0)_=+\\|qQwWeErRtTyYuUiIoOpP[{]}aAsSdDfFgGhHjJkKlL;:'\"zZxXcCvVbBnNmM,<.>/?",
    "ru": 'ёЁ1!2"3№4;5%6:7?8*9(0)_=+\\/йЙцЦуУкКеЕнНгГшШщЩзЗхХъЪфФыЫвВаАпПрРоОлЛдДжЖэЭяЯчЧсСмМиИтТьЬбБюЮ.,',
    "ua": "'ʼ1!2\"3№4;5%6:7?8*9(0)_=+ґҐйЙцЦуУкКеЕнНгГшШщЩзЗхХїЇфФіІвВаАпПрРоОлЛдДжЖєЄяЯчЧсСмМиИтТьЬбБюЮ.,",
}


def main():
    if len(sys.argv) < 1:
        return

    current = sys.argv[1]
    alt = sys.argv[2]

    action = "map"
    if current == alt:
        action = "unmap"

    default = list(LANGMAP.get(DEFAULT))
    alt = list(LANGMAP.get(alt))
    for i in range(len(default)):
        print(f"{action} global insert %§{default[i]}§ %§{alt[i]}§")


if __name__ == "__main__":
    main()

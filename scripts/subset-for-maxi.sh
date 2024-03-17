#!/usr/bin/env bash

pyftsubset assets/FiraCodeNerdFontMono-Medium.ttf \
  --output-file=assets/FiraCodeNerdFontMono-Medium.subset.ttf \
  --text=" .!ACEIGHNTYabcdefghijklmnopqrstuvwxyz" \
  --layout-features='*' \
  --glyph-names \
  --symbol-cmap \
  --notdef-glyph \
  --notdef-outline \
  --name-IDs='*' \
  --recommended-glyphs \
  --name-legacy \
  --name-languages='*'



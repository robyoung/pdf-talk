#!/usr/bin/env bash


cargo run -- --xref-type=table --output=documents/gen/mini-table.pdf create-mini
cargo run -- --xref-type=stream --output=documents/gen/mini-stream.pdf create-mini

# Maxi: LoPDF
cargo run -- \
  --xref-type=table \
  --font-type=type0 \
  --driver=lopdf \
  --output=documents/gen/maxi-table-type0-lopdf-fullset.pdf \
  create-maxi

cargo run -- \
  --xref-type=table \
  --font-type=type0 \
  --driver=lopdf \
  --subset \
  --output=documents/gen/maxi-table-type0-lopdf-subset.pdf \
  create-maxi

cargo run -- \
  --xref-type=table \
  --font-type=ttf \
  --driver=lopdf \
  --output=documents/gen/maxi-table-ttf-lopdf-fullset.pdf \
  create-maxi

cargo run -- \
  --xref-type=table \
  --font-type=ttf \
  --driver=lopdf \
  --subset \
  --output=documents/gen/maxi-table-ttf-lopdf-subset.pdf \
  create-maxi

cargo run -- \
  --xref-type=stream \
  --font-type=type0 \
  --driver=lopdf \
  --output=documents/gen/maxi-stream-type0-lopdf-fullset.pdf \
  create-maxi

cargo run -- \
  --xref-type=stream \
  --font-type=type0 \
  --driver=lopdf \
  --subset \
  --output=documents/gen/maxi-stream-type0-lopdf-subset.pdf \
  create-maxi

cargo run -- \
  --xref-type=stream \
  --font-type=ttf \
  --driver=lopdf \
  --output=documents/gen/maxi-stream-ttf-lopdf-fullset.pdf \
  create-maxi

cargo run -- \
  --xref-type=stream \
  --font-type=ttf \
  --driver=lopdf \
  --subset \
  --output=documents/gen/maxi-stream-ttf-lopdf-subset.pdf \
  create-maxi

# Maxi:PrintPDF
cargo run -- \
  --driver=printpdf \
  --output=documents/gen/maxi-printpdf-fullset.pdf \
  create-maxi

cargo run -- \
  --driver=printpdf \
  --subset \
  --output=documents/gen/maxi-printpdf-subset.pdf \
  create-maxi

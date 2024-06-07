# What even is a PDF?

This is a talk about the structure of PDF files and how they can be created.

The slide deck for the talk can be created with:

```bash
cargo run create deck deck.pdf
```

## Commands

This repository contains a number of commands for generating different PDF files.
The following command will show you what PDFs can be generated.

````bash

```bash
cargo run create
````

## Useful tools

Some tools used in the creation of this talk.

### Structure of the PDF tree

- [Inspect PDF (web UI)](https://pdfux.com/inspect-pdf/)
- [PDF Structure Viewer (CLI)](https://crates.io/crates/pdf_structure_viewer)

### qpdf

https://qpdf.readthedocs.io

Useful for exploring the structure of a PDF on the command line

```bash
qpdf --json out.pdf | bat -l json
```

Show a pretty printed version

```bash
qpdf --qdf --object-streams=disable out.pdf -
```

### allsorts

https://github.com/yeslogic/allsorts-tools

Useful for exploring and subsetting fonts.

Print the character map

```bash
allsorts cmap --font input-font.ttf
```

Subset a font with characters from a block of text

**WARNING: this seems to optimise away some information, use pyftsubset instead**

```bash
allsorts subset \
  --text "text to use for subsetting" \
  input-font.ttf output-font.ttf
```

### mutool

Useful for extracting fonts and images from PDF files.

List fonts and images

```bash
mutool info -FI input.pdf
```

Show internal PDF objects (can be browsed more easily with qpdf).

```bash
mutool show input.pdf <object id>
```

Extract all fonts and images into current directory.

```bash
mutool extract input.pdf
```

### fonttools

```bash
pip install fonttools
```

Subset a font with `pyftsubset`

```bash
pyftsubset input-font.ttf \
  --gids=<glyph-ids> \
  --layout-features='*' \
  --glyph-names \
  --symbol-cmap \
  --notdef-glyph \
  --notdef-outline \
  --name-IDs='*' \
  --recommended-glyphs \
  --name-legacy \
  --name-languages='*'
```

### Crates

- https://crates.io/crates/printpdf
- https://crates.io/crates/lopdf

## Example files

| Name                                        | Files                                                                                              |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Minimal                                     | [PDF](./pdfs/mini.pdf) \| [qPDF](./pdfs/mini.qpdf.pdf)                                             |
| Maximal streams and type0 font              | [PDF](./pdfs/maxi-type0-stream.pdf) \| [qPDF](./pdfs/maxi-type0-stream.qpdf.pdf)                   |
| Maximal no streams and ttf font             | [PDF](./pdfs/maxi-ttf-nostream.pdf) \| [qPDF](./pdfs/maxi-ttf-nostream.qpdf.pdf)                   |
| Maximal no streams, ttf font, no subsetting | [PDF](./pdfs/maxi-ttf-nostream-nosubset.pdf) \| [qPDF](./pdfs/maxi-ttf-nostream-nosubset.qpdf.pdf) |
| Web page                                    | [PDF](./pdfs/web.pdf) \| [qPDF](./pdfs/web.qpdf.pdf)                                               |

## Talk notes

### Contents

- [What is a PDF?](#what-is-a-pdf)
- [History of PDF](#history-of-pdf)
- [Basic structure](#basic-structure)
- [Document content](#document-content)
- [A complex PDF](#a-complex-pdf)

### What is a PDF?

- Portable: independent of application software, hardware and operating system.
- Document: complete description of fixed-layout flat document.
- File: everything needed to present the document can be stored within a single file.

### History

https://www.adobe.com/uk/acrobat/resources/pdf-timeline.html

- 1982 Postscript
- 1993 PDF 1.0 Text, images, pages, links, bookmarks.
- 1994 PDF 1.1 Passwords, encryption, device-independent colour.
- 1996 PDF 1.2 Forms, interactive elements, audio, compression, unicode, more advanced colour features.
- 2000 PDF 1.3 Digital signatures, ICC colour spaces, JavaScript, attachments, new annotation types.
- 2001 PDF 1.4 Accessibility (tagged PDF), format and scripting improvements.
- 2003 PDF 1.5 Multimedia objec streams (video, 3d models etc), improved encryption and format handling.
- 2004 PDF 1.6 3d artwork, OpenType font embedding, XFA (XML Forms Architecture), improved colour spaces and scripting.
- 2006 PDF 1.7 Stabilising. Improved 3d models, text, attachments, encryption and scripting.
- 2008 PDF 1.7E1
- 2008 PDF 1.7E3
- 2009 PDF 1.7E5 XFA 3.0
- 2009 PDF 1.7E6 XFA 3.1
- 2011 PDF 1.7E8 XFA 3.3
- 2008 PDF 1.7 (ISO 32000-1:2008)
- 2017 PDF 2.0 (ISO 32000-2:2020)
  - Elimination of all proprietary elements
  - Improvements across encryption, signatures, accessibility, rich media.
  - Removes XFA.

### Three documents

- One minimal PDF with one page, containing some text and minimal metadata.
- One maximal PDF with multiple pages, text with different fonts, vector graphics and images.
- A web page rendered as a PDF.

### Tools

- qpdf: useful for exploring PDF files on the command line. qpdf can render a PDF as a qpdf file which is a valid PDF
  file with additional whitespace and comments. It can also render the PDF tree as a JSON object.
- mutool: useful for extracting fonts and images from a PDF file. This is surprisingly difficult without a tool. It can
  be useful for checking for duplication or redundancy in the fonts. mutool also has commands to show the internal
  structure of a PDF file and modify it.
- allsorts: useful for exploring and subsetting fonts. It can be used to extract the character map of a font and subset
  it.

### File structure

(See: Spec 7.5 File Structure)

- Header `%PDF-1.7`
- Body
  - List of indirect objects (Spec 7.3.10) in the PDF.
  - Each object has an ID and may reference other objects by their ID.
  - Object stream (stream) stream of non-stream objects, useful for compression
  - Important objects
    - `Catalog` (dictionary) the root of the object hierarchy.
    - `Info` (dictionary) metadata about the file such as author, title, subject etc.
- Cross-reference table
  - Keeps track of the offsets of objects within the file, enabling random access to objects.
- Trailer
  - Contains pointers to important objects
  - `Size` total number of entries in the cross-reference table.
  - `Prev` points to the (previous?) cross-reference section when dealing with updated PDF files.
  - `Root` points to the catalog dictionary.
  - `Encrypt` points to the encryption dictionary. Only present if the file is encrypted.
  - `Info` points to the information dictionary.
  - `ID` two unique identifiers for the file. Mostly relevant for encryption.
- StartXRef
  - Offset of the cross-reference table.
- EOF marker `%%EOF`

### Document structure

- Catalog (dictionary) (Spec 7.7.2) the root of the object hierarchy, points to:
  - Pages (dictionary) (Spec 7.7.3.2) a node in the page tree. Often just the root page node.
    - Resources (dictionary) (Spec 7.8.3) a dictionary of resources that may be used in a content stream. For example `<</Font<</F1 2 0 R>>>>` maps the name `F1` to an indirect object reference `2 0`
    - Page (dictionary) (Spec 7.7.3.3) a leaf in the page tree.
      - Content Stream (stream or array) (Spec 7.8.2)

### Content stream

- See Table 50 in Spec 8.2 Graphics Objects for a list of operators that can be used in a content stream.
- See Spec 7.3.4 String objects for details of listeral strings `(...)`, hex strings `<...>`, and name objects `/...`.

### Problems

_Text extraction_: Extracting text from a PDF can be difficult because the text is not stored in a simple way. Text is
often shifted slightly which can be interpreted as space or vice versa.

_Font subsetting_: When merging PDF files the fonts can be duplicated or incorrectly subsetted which can result in bigger files. We see this with the PDF generator.

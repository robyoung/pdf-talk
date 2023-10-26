# What even is a PDF?

## Tasks

- [ ] Write out overall structure

## Reference
- [Chat GPT](https://chat.openai.com/c/d897ab81-1e3e-48da-822f-5a16321df8f7)
-

## Tools

- https://pdfux.com/inspect-pdf/
- https://crates.io/crates/pdf_structure_viewer

### qpdf

```bash
qpdf --json out.pdf | bat -l json
```

Show a pretty printed version
```bash
qpdf --qdf --object-streams=disable out.pdf -
```

### Crates

- https://crates.io/crates/printpdf
- https://crates.io/crates/lopdf

### Custom lopdf tools

- Create a minimal PDF
- Convert a PDF into a single page PDF showing how much of the file is used for each section.
- Resize a PDF page to a smaller frame within another page.


## The Talk

### Contents

- What will we cover
- What will we not cover
    - Content formatting in detail
    - Additional features
    - Security (encryption and signatures)
    - Forms (form inputs and javascript)
    - Multimedia (audio, video, 3D models)
    - Rich media (animations and scripted content)
    - JavaScript
    - Layers
    - Annotations
    - Reading flow
    - Accessibility features
    - Tagging and alternate text

### History

- Postscript
- Adobe
- Purpose
- Timeline?

### Tools

### The files

- One minimal PDF with one page, containing no text and minimal metadata.
- A moderate sized Benchmarking report.

### Structure of a PDF

- Header `%PDF-1.7`
- Body
    - List of objects in the PDF.
    - Each object has an ID and may reference other objects by their ID.
    - Important objects
        - `Catalog` (dictionary) the root of the object hierarchy.
        - `Info` (dictionary) metadata about the file such as author, title, subject etc.
- Cross-reference table
    - Keeps track of the offsets of objects within the file, enabling random access to objects.
- Trailer
    - `Size` total number of entries in the cross-reference table.
    - `Prev` points to the (previous?) cross-reference section when dealing with updated PDF files.
    - `Root` points to the catalog dictionary. 
    - `Encrypt` points to the encryption dictionary. Only present if the file is encrypted.
    - `Info` points to the information dictionary.
    - `ID` two unique identifiers for the file. Mostly relevant for encryption.
- EOF marker `%%EOF`

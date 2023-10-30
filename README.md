# What even is a PDF?

## Tasks

- [ ] Write out overall structure

## Reference
- [Chat GPT](https://chat.openai.com/c/d897ab81-1e3e-48da-822f-5a16321df8f7)
- [GOV.UK Accessibility](https://www.gov.uk/guidance/publishing-accessible-documents)

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

### What is a PDF?

- Portable: independent of application software, hardware and operating system.
- Document: complete description of fixed-layout flat document.
- File: everything needed to present the document can be stored within a single file.

### History

- 1982 Postscript
- 1993 PDF 1.0    Text, images, pages, links, bookmarks.
- 1994 PDF 1.1    Passwords, encryption, device-independent colour.
- 1996 PDF 1.2    Forms, interactive elements, audio, compression, unicode, more advanced colour features.
- 2000 PDF 1.3    Digital signatures, ICC colour spaces, JavaScript, attachments, new annotation types.
- 2001 PDF 1.4    Accessibility (tagged PDF), format and scripting improvements.
- 2003 PDF 1.5    Multimedia objec streams (video, 3d models etc), improved encryption and format handling.
- 2004 PDF 1.6    3d artwork, OpenType font embedding, XFA (XML Forms Architecture), improved colour spaces and scripting.
- 2006 PDF 1.7    Stabilising. Improved 3d models, text, attachments, encryption and scripting.
- 2008 PDF 1.7E1 
- 2008 PDF 1.7E3 
- 2009 PDF 1.7E5  XFA 3.0
- 2009 PDF 1.7E6  XFA 3.1
- 2011 PDF 1.7E8  XFA 3.3
- 2008 PDF 1.7 (ISO 32000-1:2008)
- 2017 PDF 2.0 (ISO 32000-2:2020)
  - Elimination of all proprietary elements
  - Improvements across encryption, signatures, accessibility, rich media.
  - Removes XFA.

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

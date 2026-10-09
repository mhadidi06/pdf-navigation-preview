# PDF Navigation Preview

**Unofficial Community Beta — Windows x64**

Unofficial community build based on the open-source PDFCraft project. Independently modified and tested. Not affiliated with, endorsed by, or supported by the ArtCraft Team or Adobe.

## Run

1. Use Windows 11 x64 and extract the entire ZIP into a new folder.
2. Open `pdf-navigation-preview.exe`, then use **Open** to select a PDF.
3. No installer, administrator access, file association or replacement of official PDFCraft is required.

The application uses independent settings and crash-recovery directories named `PDFNavigationPreview`; it does not migrate official PDFCraft/PrintCraft settings or recent-file history. User settings and documents are not included in the package.

## Navigation

- **Ctrl + mouse wheel:** zoom around the cursor within the PDF viewport.
- **Middle button + drag:** pan horizontally, vertically or diagonally.
- **Hold Space + left-button drag:** temporary hand tool; release either key/button to stop. Space remains available for typing in text fields and active editors.
- Ordinary wheel scrolling and ordinary left clicks retain their existing behavior.
- Continuous, single-page and two-page reader layouts are supported. The Organize thumbnail grid keeps its existing behavior.

Try 100%, 200% and 400% zoom, document edges, multiple pages and switching away from the window. Motion is bounded by actual scrollable document dimensions.

## Build and runtime

This repository keeps upstream Rust crate names and APIs. The preview identity is separate from its upstream version (PDFCraft 0.2.1). The release package includes `BUILD_INFO.md` with the verified runtime linkage, tests and source commit.

The v0.1 Windows x64 executable links the MSVC CRT statically. It does not import vcruntime140.dll or msvcp140.dll and requires no separate Visual C++ Redistributable installation. Its remaining DLL imports are Windows system libraries. No Microsoft runtime DLL is bundled.

This is an unsigned community test build. Windows may show an unknown-publisher/SmartScreen prompt. It inherits PDFCraft's evolving PDF rendering, editing and printing support; it is not a certified production/prepress release. Optional craft-fonts Japanese typefaces are not bundled in this build.

## Attribution and feedback

Upstream: https://github.com/storytold/pdfcraft

Independent contributor: https://github.com/mhadidi06

Original copyrights and required third-party notices are preserved in `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md` and `licenses/`. The neutral preview icon is original artwork under MIT. Protected ArtCraft graphics are excluded from the modified application and package.

For a test report, include Windows version, preview version, PDF page count, zoom/layout, steps and expected/observed behavior. Do not post private PDFs, recent-file lists, credentials or personal settings.

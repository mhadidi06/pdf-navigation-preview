# Changelog

## 0.1.0 — Community Beta

- Cursor-centered Ctrl + mouse-wheel zoom within the PDF reader viewport.
- Immediate, bounded middle-button drag pan using the existing ScrollArea.
- Temporary Space + left-button drag pan with Grab/Grabbing cursors and editing-input isolation.
- Cancellation on button/key release, document changes and focus loss.
- Fixed initial numeric zoom options before the first viewport layout.
- Independent preview name, neutral original icon, Windows identifiers, settings and recovery namespace.
- Preserved upstream PDF functionality, copyright notices and contributor credits.

This preview is based on PDFCraft 0.2.1. Its community version number does not identify an official upstream release.

- Link the Windows MSVC CRT statically; no VC runtime DLLs or installer are distributed.
- Preserve dependency and Rust standard-library copyright/license notices in the portable package.

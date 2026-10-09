# PDF Navigation Preview v0.1 — Windows x64

**Unofficial Community Beta — Windows x64**

Unofficial community build based on the open-source PDFCraft project. Independently modified and tested. Not affiliated with, endorsed by, or supported by the ArtCraft Team or Adobe.

This portable preview invites testing of cursor-centered Ctrl-wheel zoom, middle-button drag pan and Space + left-button drag pan in continuous, single-page and two-page PDF reading layouts. It retains the original PDF functionality and uses independent application identity and settings.

Extract the ZIP and run `pdf-navigation-preview.exe`. Read `README.md` for navigation and runtime requirements, `BUILD_INFO.md` for validation, and `SHA256SUMS.txt` for file checksums.

The build is unsigned and experimental. Manual testing on different Windows systems and production PDF/prepress workflows is still needed. No installer or automatic upstream update feed is included. Do not publish private documents or settings in feedback.

Validation: formatting, Cargo check, workspace Clippy and 929 automated tests pass (4 pre-existing tests require external fixtures/network or a timing probe). The Windows x64 static-CRT Release build starts successfully and opens/renders a synthetic four-page PDF. Native physical mouse testing is pending because the validation desktop was locked; the navigation input sequences pass integration tests. No separate Visual C++ Redistributable is required for this executable.

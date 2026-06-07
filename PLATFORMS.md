# Platform Support

layered targets three desktop platforms. This document defines the support
matrix, known constraints, and the policy for adding new platform-specific
behavior.

---

## Supported Platforms

| Platform | Status | Runtime | Notes |
|----------|--------|---------|-------|
| Linux (X11) | Supported | WebKitGTK via Dioxus/wry | Install system packages first |
| Linux (Wayland) | Supported (via XWayland) | WebKitGTK | Native Wayland support follows wry upstream |
| macOS | Supported | WKWebView | No extra packages required |
| Windows | Supported | WebView2 | Requires WebView2 runtime (bundled in recent Windows) |

---

## Linux System Packages

Before building or running on Debian/Ubuntu-based systems:

```sh
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev
```

Fedora/RHEL equivalent:

```sh
sudo dnf install webkit2gtk4.1-devel gtk3-devel
```

---

## Platform-Specific Behavior

### Keyboard Modifiers

On macOS the Cmd key is used where Linux/Windows use Ctrl. The keyboard
handler in `layered-desktop/src/keyboard.rs` normalises this automatically
using the `keyboard-types` crate's modifier detection.

### File Dialogs

Native file dialogs (`rfd` crate) use the platform's standard picker:

- Linux: GTK portal or fallback file chooser
- macOS: NSOpenPanel / NSSavePanel
- Windows: IFileOpenDialog / IFileSaveDialog

If the portal is unavailable on a headless Linux system, `rfd` may silently
return no file. This is documented behavior.

### Config Directory

Settings are stored in platform-appropriate locations (RFC-036):

| Platform | Path |
|----------|------|
| Linux | `~/.config/layered/settings.toml` |
| macOS | `~/Library/Application Support/layered/settings.toml` |
| Windows | `%APPDATA%\layered\settings.toml` |

---

## Policy for Platform-Specific Code

All platform-specific behavior must be isolated in `layered-desktop`.
`layered-core` and `layered-ui` must compile and test on any host without
GUI libraries (RFC-001, RFC-010 boundary rule).

Platform-specific workarounds should be documented inline with a reference
to the upstream issue where applicable.

---

## Release Platform Matrix

A release may only claim support for a platform that has passed the smoke
test checklist in `docs/smoke-tests.md`. Platforms tested and passing are
listed in the release notes under "Platform Notes".

---

## Known Constraints

- No mobile (iOS/Android) support.
- No browser/web app distribution.
- No ARM/RISC-V testing in initial releases (community contributions welcome).
- Wayland: some compositor-specific window management edge cases may apply.
  Report issues with your compositor name and version.
- Unsigned macOS builds will show a Gatekeeper warning. See
  `RELEASE_CHECKLIST.md` for verification instructions.

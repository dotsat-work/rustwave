# Rustwave

<p align="center">
  <img src="assets/rustwave.png" alt="Rustwave logo" width="480"/>
</p>

A lightweight Linux desktop wrapper for [Brain.fm](https://www.brain.fm/app), built with [Tauri 2](https://tauri.app/).

Brain.fm is a music app designed to improve focus, relaxation, and sleep. Rustwave gives it a native desktop experience on Linux — no browser tab needed. Your session is persisted across launches, so you log in once and stay logged in.

---

## Features

- Native desktop window for Brain.fm on Linux
- Persistent login session (no re-authentication on every launch)
- Packages for Debian/Ubuntu (`.deb`) and Fedora/RHEL (`.rpm`) with GStreamer dependencies declared automatically
- Raw binary available for any Linux distribution

---

## Installation

### Debian / Ubuntu / Mint
```bash
sudo apt install ./rustwave_0.1.0_amd64.deb
```

### Fedora / RHEL / openSUSE
```bash
sudo dnf install ./rustwave-0.1.0-1.x86_64.rpm
```

### Arch / EndeavourOS / Manjaro

Install using the provided `pkg/` directory, which builds a native pacman package:

**1. Install build dependencies (first time only):**
```bash
sudo pacman -S webkit2gtk-4.1 gst-plugins-base gst-plugins-good gst-plugins-bad gst-libav
```

**2. Build and install the package:**
```bash
cd pkg
makepkg -sf
sudo pacman -U rustwave-0.1.0-1-x86_64.pkg.tar.zst
```

This installs:
- The binary to `/usr/bin/rustwave` (available system-wide on PATH)
- The app icon to `/usr/share/icons/hicolor/256x256/apps/`
- A `.desktop` file to `/usr/share/applications/` so Rustwave appears in your app launcher with the correct icon

**To update after rebuilding:**
```bash
cargo tauri build
cp src-tauri/target/release/rustwave pkg/
cd pkg && makepkg -sf
sudo pacman -U rustwave-0.1.0-1-x86_64.pkg.tar.zst
```

---

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Tauri CLI](https://tauri.app/start/): `cargo install tauri-cli`
- System dependencies for Tauri on Linux:

**Debian/Ubuntu:**
```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad libgstreamer1.0-0
```

**Arch/EndeavourOS:**
```bash
sudo pacman -S webkit2gtk-4.1 gtk3 libayatana-appindicator librsvg \
  gst-plugins-base gst-plugins-good gst-plugins-bad gst-libav patchelf
```

### Development

```bash
git clone https://github.com/AlexTLDR/rustwave.git
cd rustwave
cargo tauri dev
```

### Production Build

```bash
cargo tauri build
```

This produces:
- `src-tauri/target/release/rustwave` — raw binary
- `src-tauri/target/release/bundle/deb/rustwave_0.1.0_amd64.deb`
- `src-tauri/target/release/bundle/rpm/rustwave-0.1.0-1.x86_64.rpm`

---

## Contributing

Contributions are welcome! If you have ideas, bug reports, or improvements, feel free to:

- Open an [issue](https://github.com/AlexTLDR/rustwave/issues)
- Submit a [pull request](https://github.com/AlexTLDR/rustwave/pulls)

Please keep PRs focused and describe what problem they solve.

---

## License

This project is licensed under the [MIT License](LICENSE).

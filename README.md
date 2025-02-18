# Open Media Utils (OMU) 🎥🎵🖼️

**Open Media Utils (OMU)** is a powerful command-line tool designed for efficient multimedia file processing. Built in Rust, it leverages `clap` for argument handling and `FFmpeg` for advanced video and audio operations. Whether you're converting files, editing videos, or processing images, OMU has you covered! 🚀

---

## Features ✨

### 🎯 **File Conversion**

- Convert between various multimedia formats:
  - Audio: `MP3` ↔ `WAV`
  - Video: `MP4` ↔ `WEBM`
  - Image: `PNG` ↔ `JPG`, `WEBP` ↔ `GIF`

### 🎥 **Video Utilities**

- Extract audio from videos.
- Mute video audio.
- Replace audio in a video.
- Trim videos (cut start or end).
- Cut segments from videos.
- Combine videos side-by-side or vertically.

### 🖼️ **Image Utilities**

- Overlay images (respecting alpha channels).
- Combine images horizontally or vertically.
- Apply filters (brightness, contrast, grayscale, blur).
- Reshape images (circle, square, rounded corners).
- Create videos from images.

### 🎵 **Audio Utilities**

- Combine multiple audio files.
- Adjust audio volume.

### 📁 **Supported extensions**

- **Audio:**
  - `MP3`, `WAV`, `AAC`, `FLAC`, `OGG`, `OPUS`, `M4A`
- **Video:**
  - `MP4`, `WEBM`, `MKV`, `AVI`, `MOV`, `MPEG`, `MPEGTS`
- **Image:**
  - `JPG`, `PNG`, `WEBP`, `GIF`, `BMP`, `JPEG`, `TIFF`, `SVG`, `ICO`, `ICNS`

---

## Installation ⚙️

### Prerequisites

- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
- **FFmpeg**: Install FFmpeg and ensure it's available in your system PATH.

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/sammwyy/omu.git
   cd omu
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the binary:

   ```bash
   ./target/release/omu --help
   ```

4. (Optional) Add the binary to your PATH for global access:

   ```bash
   sudo cp ./target/release/omu /usr/local/bin/omu
   ```

---

## Usage 🛠️

### General Syntax

```bash
./omu <command> <subcommand> [arguments]
```

### Examples

#### Convert MP3 to WAV

```bash
./omu convert -i input.mp3 -o output.wav
```

#### Extract Audio from Video

```bash
./omu video extract-audio -i input.mp4 -o output.wav
```

#### Overlay Images

```bash
./omu image overlay --base base.png --overlay overlay.png -o result.png --x 50 --y 50
```

#### Combine Audios

```bash
./omu audio combine -i audio1.mp3 -i audio2.wav -o combined.mp3
```

---

## Documentation 📚

For detailed documentation on all commands and arguments, check out the [CLI Documentation](./docs/cli.md).

---

## Contributing 🤝

We welcome contributions! Here's how you can help:

1. Fork the repository.
2. Create a new branch for your feature or fix:

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. Commit your changes:

   ```bash
   git commit -m "Add your feature"
   ```

4. Push to your branch:

   ```bash
   git push origin feature/your-feature-name
   ```

5. Open a pull request.

---

## License 📜

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments 🙏

- **FFmpeg**: For providing the backbone of multimedia processing.
- **Clap**: For command-line argument parsing.
- **Image**: For image processing.
- **Rust Community**: For creating an amazing ecosystem of tools and libraries.

---

## Support 💖

If you find this project useful, please consider giving it a ⭐️ on GitHub! Your support helps us grow and improve.

---

Happy multimedia processing! 🎉🎬🎶

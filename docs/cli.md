# Open Media Utils (OMU) - CLI Documentation

**Open Media Utils (OMU)** is a command-line tool designed for efficient multimedia file processing. Built in Rust, it uses `clap` for argument handling and `FFmpeg` for advanced video and audio operations. Below is the complete CLI documentation.

---

## General Usage

The basic CLI structure is:

```bash
./omu <command> <subcommand> [arguments]
```

---

## Available Commands

### 1. **File Conversion**

Convert multimedia files between supported formats.

#### Subcommand

```bash
./omu convert -i <input> -o <output> [--extra-args <args>]
```

#### Arguments

- `-i, --input`: Path to the input file.
- `-o, --output`: Path to the output file.
- `--extra-args`: Additional FFmpeg arguments (optional).

#### Examples

```bash
# Convert MP3 to WAV
./omu convert -i audio.mp3 -o audio.wav

# Convert MP4 to WEBM
./omu convert -i video.mp4 -o video.webm
```

---

### 2. **Video Utilities**

Perform advanced operations on video files.

#### Subcommands

1. **Extract audio**:

   ```bash
   ./omu video extract-audio -i <input> -o <output>
   ```

2. **Mute audio**:

   ```bash
   ./omu video mute -i <input> -o <output>
   ```

3. **Trim video**:

   ```bash
   ./omu video trim -i <input> -o <output> [--start <time>] [--end <time>]
   ```

4. **Cut segment**:

   ```bash
   ./omu video cut -i <input> -o <output> --start <time> --end <time>
   ```

5. **Replace audio**:

   ```bash
   ./omu video replace-audio --video <video> --audio <audio> -o <output>
   ```

6. **Combine videos**:

   ```bash
   ./omu video combine -i <input1> -i <input2> -o <output> --mode <mode>
   ```

#### Common Arguments

- `-i, --input`: Path to the input file.
- `-o, --output`: Path to the output file.
- `-s --start`: Start time (in seconds or `HH:MM:SS` format).
- `-e --end`: End time (in seconds or `HH:MM:SS` format).
- `-m --mode`: Combination mode (`horizontal`, `vertical`, `overlay`).

#### Examples

```bash
# Extract audio from a video
./omu video extract-audio -i video.mp4 -o audio.mp3

# Trim video from 00:01:00 to 00:02:00
./omu video trim -i video.mp4 -o trimmed.mp4 --start 00:01:00 --end 00:02:00
```

---

### 3. **Image Utilities**

Process and manipulate images.

#### Subcommands

1. **Overlay images**:

   ```bash
   ./omu image overlay --base <base> --overlay <overlay> -o <output> --x <x> --y <y>
   ```

2. **Combine images**:

   ```bash
   ./omu image combine --input1 <input1> --input2 <input2> -o <output> --mode <mode>
   ```

3. **Apply filters**:

   ```bash
   ./omu image filter --input <input> -o <output> --filter <filter> [--intensity <value>]
   ```

4. **Reshape image**:

   ```bash
   ./omu image reshape --input <input> -o <output> --shape <shape> [--radius <value>]
   ```

5. **Create video from image**:

   ```bash
   ./omu image create-video --input <input> -o <output> --duration <seconds>
   ```

#### Common Arguments

- `--input`: Path to the input image.
- `-o, --output`: Path to the output image.
- `--mode`: Combination mode (`horizontal`, `vertical`).
- `--filter`: Filter type (`grayscale`, `brightness`, `contrast`, `blur`).
- `--intensity`: Filter intensity (optional).
- `--shape`: Reshape type (`circle`, `square`, `rounded`).
- `--radius`: Border radius for rounded shape (optional).
- `--duration`: Video duration in seconds.

#### Examples

```bash
# Combine two images horizontally
./omu image combine --input1 img1.png --input2 img2.png -o result.png --mode horizontal

# Apply blur filter
./omu image filter --input input.png -o output.png --filter blur --intensity 2.0
```

---

### 4. **Audio Utilities**

Manipulate audio files.

#### Subcommands

1. **Combine audios**:

   ```bash
   ./omu audio combine -i <input1> -i <input2> -o <output>
   ```

2. **Change volume**:

   ```bash
   ./omu audio volume -i <input> -o <output> --volume <value>
   ```

#### Common Arguments

- `-i, --input`: Path to the input file.
- `-o, --output`: Path to the output file.
- `--volume`: Volume multiplier (e.g., `0.5` for half volume, `2.0` for double).

#### Examples

```bash
# Combine two audio files
./omu audio combine -i audio1.mp3 -i audio2.mp3 -o combined.mp3

# Increase volume by 2x
./omu audio volume -i input.mp3 -o output.mp3 --volume 2.0
```

---

## Advanced Usage Examples

1. **Create a GIF from a video**:

   ```bash
   ./omu convert -i video.mp4 -o animation.gif
   ```

2. **Extract audio and change its volume**:

   ```bash
   ./omu video extract-audio -i video.mp4 -o audio.wav
   ./omu audio volume -i audio.wav -o loud_audio.wav --volume 1.5
   ```

3. **Create a video with rounded corners**:

   ```bash
   ./omu image reshape --input image.png -o rounded.png --shape rounded --radius 50
   ./omu image create-video --input rounded.png -o video.mp4 --duration 10
   ```

---

## Notes

- **FFmpeg**: Ensure FFmpeg is installed and available in your PATH.
- **Supported Formats**: OMU supports all formats compatible with FFmpeg and the `image` crate in Rust.
- **Performance**: Built in Rust, OMU is fast and resource-efficient.

---

This CLI documentation provides a comprehensive guide to using Open Media Utils. For more details, refer to the project's README or source code. 🚀

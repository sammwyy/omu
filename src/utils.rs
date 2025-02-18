use anyhow::{Context, Result};
use native_dialog::FileDialog;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

pub fn run_ffmpeg_command(args: &[&str]) -> Result<()> {
    let output = Command::new("ffmpeg")
        .args(args)
        .output()
        .context("Failed to execute ffmpeg command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("FFmpeg error: {}", stderr);
    }
    Ok(())
}

pub fn verify_magic_bytes(file_path: &Path, expected_magic: &[u8]) -> Result<bool, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0; expected_magic.len()];
    file.read_exact(&mut buffer)?;
    Ok(buffer == expected_magic)
}

pub fn audio_codec_by_ext(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "mp3" => Some("libmp3lame"),
        "wav" => Some("pcm_s16le"),
        "aac" => Some("aac"),
        "flac" => Some("flac"),
        "ogg" => Some("libvorbis"),
        "opus" => Some("libopus"),
        "m4a" => Some("alac"),
        _ => None,
    }
}

pub fn ext_by_filename(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .map(|ext| ext.to_str().unwrap_or("").to_string())
}

pub fn create_temp_file(extension: &String) -> PathBuf {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let temp_file_with_ext = temp_file.path().with_extension(extension);
    temp_file_with_ext
}

#[derive(Clone, Debug)]
pub enum FileType {
    Video,
    Image,
    Audio,
}

const VIDEO_EXTENSIONS: [&str; 7] = ["mp4", "webm", "mkv", "avi", "mov", "mpeg", "mpegts"];
const IMAGE_EXTENSIONS: [&str; 10] = [
    "jpg", "png", "webp", "gif", "bmp", "jpeg", "tiff", "svg", "ico", "icns",
];
const AUDIO_EXTENSIONS: [&str; 7] = ["mp3", "wav", "aac", "flac", "ogg", "opus", "m4a"];

pub fn open_file_dialog(file_type: FileType) -> Option<PathBuf> {
    let mut dialog = FileDialog::new().set_title("Select a file");

    match file_type {
        FileType::Video => dialog = dialog.add_filter("Video Files", &VIDEO_EXTENSIONS),
        FileType::Image => dialog = dialog.add_filter("Image Files", &IMAGE_EXTENSIONS),
        FileType::Audio => dialog = dialog.add_filter("Audio Files", &AUDIO_EXTENSIONS),
    }

    dialog.show_save_single_file().unwrap_or(None)
}

pub fn get_file_arg(file_type: FileType, arg: &Option<PathBuf>) -> Result<PathBuf> {
    match arg {
        Some(path) => Ok(path.to_path_buf()),
        None => open_file_dialog(file_type).context("Failed to open file dialog"),
    }
}

pub fn file_type_from_extension(extension: &str) -> Result<FileType> {
    match extension {
        "mp4" | "webm" | "mkv" | "avi" | "mov" | "mpeg" | "mpegts" => Ok(FileType::Video),
        "jpg" | "png" | "webp" | "gif" | "bmp" | "jpeg" | "tiff" | "svg" | "ico" | "icns" => {
            Ok(FileType::Image)
        }
        "mp3" | "wav" | "aac" | "flac" | "ogg" | "opus" | "m4a" => Ok(FileType::Audio),
        _ => Err(anyhow::anyhow!("Unsupported file type: {}", extension)),
    }
}

use anyhow::{Context, Result};
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

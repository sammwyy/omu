use anyhow::Result;
use clap::{Args, Subcommand};
use std::{path::PathBuf, process::Command};

use crate::utils::run_ffmpeg_command;

#[derive(Subcommand)]
pub enum AudioCommand {
    /// Combine multiple audio files
    Combine(CombineAudioArgs),

    /// Change audio volume
    Volume(VolumeArgs),
}

impl AudioCommand {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Combine(cmd) => cmd.execute(),
            Self::Volume(cmd) => cmd.execute(),
        }
    }
}

#[derive(Args)]
pub struct CombineAudioArgs {
    /// Input audio files
    #[arg(short, long, required = true)]
    pub inputs: Vec<PathBuf>,

    /// Output file path
    #[arg(short, long)]
    pub output: PathBuf,
}

impl CombineAudioArgs {
    pub fn execute(&self) -> Result<()> {
        let mut ffmpeg_args = vec!["-y"]; // Overwrite output

        // Add inputs
        for input in &self.inputs {
            ffmpeg_args.push("-i");
            ffmpeg_args.push(input.to_str().unwrap());
        }

        // Complex filter to concatenate audios
        ffmpeg_args.push("-filter_complex");
        let filter = (0..self.inputs.len())
            .map(|i| format!("[{}:a]", i))
            .collect::<Vec<_>>()
            .join("")
            + &format!("concat=n={}:v=0:a=1[out]", self.inputs.len());
        ffmpeg_args.push(&filter);

        // Map output
        ffmpeg_args.push("-map");
        ffmpeg_args.push("[out]");

        // Output file
        ffmpeg_args.push(self.output.to_str().unwrap());

        run_ffmpeg_command(&ffmpeg_args)
    }
}

#[derive(Args)]
pub struct VolumeArgs {
    /// Input audio file
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Volume multiplier (e.g., 0.5 for half volume, 2.0 for double)
    #[arg(short, long)]
    pub volume: f32,
}

impl VolumeArgs {
    pub fn execute(&self) -> Result<()> {
        let args = [
            "-i",
            self.input.to_str().unwrap(),
            "-filter:a",
            &format!("volume={}", self.volume),
            "-y",
            self.output.to_str().unwrap(),
        ];

        run_ffmpeg_command(&args)
    }
}

pub fn get_audio_duration(path: &PathBuf) -> Result<f64> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_str().unwrap(),
        ])
        .output()?;

    let duration_str = String::from_utf8(output.stdout)?;
    duration_str.trim().parse::<f64>().map_err(Into::into)
}

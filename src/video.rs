use crate::utils::{audio_codec_by_ext, create_temp_file, ext_by_filename, run_ffmpeg_command};
use anyhow::Result;
use clap::{Args, Subcommand};
use std::{fs::File, io::Write, path::PathBuf};
use tempfile::NamedTempFile;

#[derive(Subcommand)]
pub enum VideoCommand {
    /// Extract audio from video
    ExtractAudio(ExtractAudioArgs),

    /// Mute video audio
    Mute(MuteVideoArgs),

    /// Trim video (cut start or end)
    Trim(TrimArgs),

    /// Cut video (remove a segment between two points)
    Cut(CutArgs),

    /// Replace audio in video
    ReplaceAudio(ReplaceAudioArgs),

    /// Combine video with another video or image
    Combine(CombineVideoArgs),
}

impl VideoCommand {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::ExtractAudio(cmd) => cmd.execute(),
            Self::Mute(cmd) => cmd.execute(),
            Self::Trim(cmd) => cmd.execute(),
            Self::Cut(cmd) => cmd.execute(),
            Self::ReplaceAudio(cmd) => cmd.execute(),
            Self::Combine(cmd) => cmd.execute(),
        }
    }
}

#[derive(Args)]
pub struct ExtractAudioArgs {
    #[arg(short, long)]
    pub input: PathBuf,
    #[arg(short, long)]
    pub output: PathBuf,
}

impl ExtractAudioArgs {
    pub fn execute(&self) -> Result<()> {
        let output_path = self.output.to_str().unwrap();
        let output_path_buf = PathBuf::from(output_path);
        let output_ext = output_path_buf
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let codec = audio_codec_by_ext(output_ext).unwrap_or("copy");

        let mut args = vec![
            "-i",
            self.input.to_str().unwrap(),
            "-vn",
            "-acodec",
            codec,
            "-y",
            output_path,
        ];

        if codec == "pcm_s16le" {
            args.extend_from_slice(&["-ar", "44100", "-ac", "2"]);
        }

        run_ffmpeg_command(&args)
    }
}

#[derive(Args)]
pub struct MuteVideoArgs {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

impl MuteVideoArgs {
    fn execute(&self) -> Result<()> {
        let args = [
            "-i",
            self.input.to_str().unwrap(),
            "-an", // Disable audio
            "-c:v",
            "copy",
            "-y",
            self.output.to_str().unwrap(),
        ];
        run_ffmpeg_command(&args)
    }
}

#[derive(Args)]
pub struct TrimArgs {
    #[arg(short, long)]
    pub input: PathBuf,
    #[arg(short, long)]
    pub output: PathBuf,
    #[arg(short, long, help = "Start time in seconds or HH:MM:SS format")]
    pub start: Option<String>,
    #[arg(short, long, help = "End time in seconds or HH:MM:SS format")]
    pub end: Option<String>,
}

impl TrimArgs {
    pub fn execute(&self) -> Result<()> {
        let mut args = vec!["-i", self.input.to_str().unwrap()];

        if let Some(start) = &self.start {
            args.push("-ss");
            args.push(start);
        }

        if let Some(end) = &self.end {
            args.push("-to");
            args.push(end);
        }

        args.extend(&["-c", "copy", "-y", self.output.to_str().unwrap()]);

        run_ffmpeg_command(&args)
    }
}

#[derive(Args)]
pub struct CutArgs {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
    #[arg(short, long, help = "Start cut point in seconds or HH:MM:SS format")]
    start: String,
    #[arg(short, long, help = "End cut point in seconds or HH:MM:SS format")]
    end: String,
}

impl CutArgs {
    fn execute(&self) -> Result<()> {
        let start = &self.start;
        let end = &self.end;
        let input_path = self.input.to_str().unwrap();
        let output_path = self.output.to_str().unwrap();

        let extension = ext_by_filename(input_path).unwrap_or_else(|| "mp4".to_string());

        let temp_file_1_with_ext = create_temp_file(&extension);
        let temp_file_2_with_ext = create_temp_file(&extension);

        let args1 = [
            "-i",
            input_path,
            "-t",
            start,
            "-c",
            "copy",
            "-y",
            temp_file_1_with_ext.to_str().unwrap(),
        ];
        run_ffmpeg_command(&args1)?;

        let args2 = [
            "-i",
            input_path,
            "-ss",
            end,
            "-c",
            "copy",
            "-y",
            temp_file_2_with_ext.to_str().unwrap(),
        ];
        run_ffmpeg_command(&args2)?;

        let concat_file = NamedTempFile::new()?;
        let concat_file_path = concat_file.path().to_str().unwrap();
        let mut file = File::create(concat_file_path)?;

        writeln!(file, "file '{}'", temp_file_1_with_ext.to_str().unwrap())?;
        writeln!(file, "file '{}'", temp_file_2_with_ext.to_str().unwrap())?;

        let concat_args = [
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            concat_file_path,
            "-c",
            "copy",
            "-y",
            output_path,
        ];
        run_ffmpeg_command(&concat_args)?;
        Ok(())
    }
}

#[derive(Args)]
pub struct ReplaceAudioArgs {
    #[arg(short, long)]
    video: PathBuf,
    #[arg(short, long)]
    audio: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

impl ReplaceAudioArgs {
    fn execute(&self) -> Result<()> {
        let audio_ext = self
            .audio
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        let codec = audio_codec_by_ext(audio_ext).unwrap_or("aac");

        let mut args = vec![
            "-i",
            self.video.to_str().unwrap(),
            "-i",
            self.audio.to_str().unwrap(),
            "-c:v",
            "copy",
            "-c:a",
            codec,
            "-map",
            "0:v:0",
            "-map",
            "1:a:0",
            "-shortest",
            "-y",
            self.output.to_str().unwrap(),
        ];

        if codec == "pcm_s16le" {
            args.extend_from_slice(&["-ar", "44100", "-ac", "2"]);
        } else {
            args.extend_from_slice(&["-b:a", "192k"]);
        }

        run_ffmpeg_command(&args)
    }
}

#[derive(Args)]
pub struct CombineVideoArgs {
    #[arg(short, long)]
    inputs: Vec<PathBuf>,
    #[arg(short, long)]
    output: PathBuf,
    #[arg(short, long, help = "Combine mode: horizontal, vertical, or overlay")]
    mode: String,
}

impl CombineVideoArgs {
    fn execute(&self) -> Result<()> {
        if self.inputs.is_empty() {
            anyhow::bail!("At least one input file is required.");
        }

        let video_filter = match self.mode.as_str() {
            "horizontal" => {
                let mut filter = String::new();
                for (i, _) in self.inputs.iter().enumerate() {
                    // Scale videos vertical
                    filter.push_str(&format!("[{}:v]scale=-1:ih[{}];", i, i));
                }

                // Stack horizontally
                let filter_parts: Vec<String> =
                    (0..self.inputs.len()).map(|i| format!("[{}]", i)).collect();
                filter.push_str(&filter_parts.join(""));
                filter.push_str("hstack[v]");
                filter
            }
            "vertical" => {
                let mut filter = String::new();
                for (i, _) in self.inputs.iter().enumerate() {
                    // Scale videos horizontal
                    filter.push_str(&format!("[{}:v]scale=iw:{}[{}];", i, self.inputs.len(), i));
                }

                // Stack vertically
                let filter_parts: Vec<String> =
                    (0..self.inputs.len()).map(|i| format!("[{}]", i)).collect();
                filter.push_str(&filter_parts.join(""));
                filter.push_str("vstack[v]");
                filter
            }
            "overlay" => {
                if self.inputs.len() != 2 {
                    anyhow::bail!("Overlay mode requires exactly two input files.");
                }
                format!("[0:v][1:v]overlay=0:0[v]")
            }
            _ => anyhow::bail!("Invalid combine mode. Use horizontal, vertical, or overlay"),
        };

        // Audio filter
        let audio_filter = (0..self.inputs.len())
            .map(|i| format!("[{}:a]", i)) // Add audio tracks
            .collect::<Vec<String>>()
            .join("")
            + "amix=inputs="
            + &self.inputs.len().to_string()
            + ":duration=longest[a]";

        let mut args = Vec::new();

        // Add input files
        for input in &self.inputs {
            args.push("-i");
            args.push(input.to_str().unwrap());
        }

        // Add complex filter for video and audio inputs
        let combined_filter = format!("{},{}", video_filter, audio_filter);
        args.push("-filter_complex");
        args.push(&combined_filter); // Combine filters

        // Map output
        args.push("-map");
        args.push("[v]");
        args.push("-map");
        args.push("[a]");
        args.push("-y");
        args.push(self.output.to_str().unwrap());

        run_ffmpeg_command(&args)
    }
}

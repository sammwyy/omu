use anyhow::Result;
use clap::Args;
use std::path::Path;

use crate::utils::run_ffmpeg_command;

pub fn convert_file(input: &Path, output: &Path, extra_args: Option<&str>) -> Result<()> {
    let mut args = vec![
        "-i",
        input.to_str().unwrap(),
        "-y", // Overwrite output file
    ];

    if let Some(extra) = extra_args {
        args.extend(extra.split_whitespace());
    }

    args.push(output.to_str().unwrap());

    run_ffmpeg_command(&args)
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Input file path
    #[arg(short, long)]
    pub input: std::path::PathBuf,

    /// Output file path
    #[arg(short, long)]
    pub output: std::path::PathBuf,

    /// Additional FFmpeg parameters
    #[arg(short, long)]
    pub extra_args: Option<String>,
}

impl ConvertArgs {
    pub fn execute(&self) -> anyhow::Result<()> {
        convert_file(&self.input, &self.output, self.extra_args.as_deref())
    }
}

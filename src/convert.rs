use anyhow::Result;
use clap::Args;
use std::path::{Path, PathBuf};

use crate::utils::{ext_by_filename, file_type_from_extension, get_file_arg, run_ffmpeg_command};

#[derive(Args)]
pub struct ConvertArgs {
    /// Input file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Additional FFmpeg parameters
    #[arg(short, long)]
    pub extra_args: Option<String>,
}

impl ConvertArgs {
    pub fn execute(&self) -> anyhow::Result<()> {
        let ext = ext_by_filename(self.input.to_str().unwrap()).unwrap();
        let file_type = file_type_from_extension(ext.as_str())?;
        let output = get_file_arg(file_type.clone(), &self.output)?;

        convert_file(&self.input, &output, self.extra_args.as_deref())
    }
}

pub fn convert_file(input: &Path, output: &PathBuf, extra_args: Option<&str>) -> Result<()> {
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

use clap::{Parser, Subcommand};

use omu::audio;
use omu::convert;
use omu::image;
use omu::video;

#[derive(Parser)]
#[command(name = "Open Media Utils")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn execute(&self) -> anyhow::Result<()> {
        self.command.execute()
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// File format conversions
    Convert(convert::ConvertArgs),

    /// Video manipulation utilities
    Video(VideoCommandWrapper),

    /// Image processing utilities
    Image(ImageCommandWrapper),

    /// Audio manipulation utilities
    Audio(AudioCommandWrapper),
}

impl Commands {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Convert(cmd) => cmd.execute(),
            Self::Video(cmd) => cmd.execute(),
            Self::Image(cmd) => cmd.execute(),
            Self::Audio(cmd) => cmd.execute(),
        }
    }
}

// Wrapper para VideoCommand
#[derive(Parser)]
pub struct VideoCommandWrapper {
    #[command(subcommand)]
    pub command: video::VideoCommand,
}

impl VideoCommandWrapper {
    fn execute(&self) -> anyhow::Result<()> {
        self.command.execute()
    }
}

// Wrapper para ImageCommand
#[derive(Parser)]
pub struct ImageCommandWrapper {
    #[command(subcommand)]
    pub command: image::ImageCommand,
}

impl ImageCommandWrapper {
    fn execute(&self) -> anyhow::Result<()> {
        self.command.execute()
    }
}

// Wrapper para AudioCommand
#[derive(Parser)]
pub struct AudioCommandWrapper {
    #[command(subcommand)]
    pub command: audio::AudioCommand,
}

impl AudioCommandWrapper {
    fn execute(&self) -> anyhow::Result<()> {
        self.command.execute()
    }
}

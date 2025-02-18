use anyhow::Result;
use clap::{Args, Subcommand};
use image::{imageops, DynamicImage, GenericImageView, RgbImage, RgbaImage};
use std::path::PathBuf;

use crate::utils::{get_file_arg, FileType};

#[derive(Subcommand)]
pub enum ImageCommand {
    /// Overlay one image on top of another
    Overlay(OverlayArgs),

    /// Combine images side by side or vertically
    Combine(CombineArgs),

    /// Apply filters to an image
    Filter(FilterArgs),

    /// Change the shape of an image
    Reshape(ReshapeArgs),

    /// Create a video from an image
    CreateVideo(CreateVideoArgs),
}

impl ImageCommand {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Overlay(cmd) => cmd.execute(),
            Self::Combine(cmd) => cmd.execute(),
            Self::Filter(cmd) => cmd.execute(),
            Self::Reshape(cmd) => cmd.execute(),
            Self::CreateVideo(cmd) => cmd.execute(),
        }
    }
}

#[derive(Args)]
pub struct OverlayArgs {
    #[arg(short, long)]
    pub input: PathBuf,
    #[arg(short = 'l', long)]
    pub overlay: PathBuf,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(long, help = "X position for overlay", default_value_t = 0)]
    pub x: i64,
    #[arg(long, help = "Y position for overlay", default_value_t = 0)]
    pub y: i64,
}

impl OverlayArgs {
    pub fn execute(&self) -> Result<()> {
        let base = image::open(&self.input)?;
        let overlay = image::open(&self.overlay)?;
        let result = overlay_images(&base, &overlay, self.x, self.y);

        // Get output or prompt for one
        let output = get_file_arg(FileType::Image, &self.output)?;
        result.save(&output)?;
        Ok(())
    }
}

pub fn overlay_images(base: &DynamicImage, overlay: &DynamicImage, x: i64, y: i64) -> DynamicImage {
    let mut result = base.clone();
    imageops::overlay(&mut result, overlay, x, y);
    result
}

#[derive(Args)]
pub struct CombineArgs {
    #[arg(short, long)]
    inputs: Vec<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long, help = "Combine mode: horizontal or vertical")]
    mode: String,
}

impl CombineArgs {
    fn execute(&self) -> Result<()> {
        if self.inputs.is_empty() {
            anyhow::bail!("At least one input file is required.");
        }

        let mut images: Vec<DynamicImage> = Vec::new();
        for input in &self.inputs {
            let img = image::open(input)?;
            images.push(img);
        }

        let result = match self.mode.as_str() {
            "horizontal" => combine_images_horizontal(&images),
            "vertical" => combine_images_vertical(&images),
            _ => anyhow::bail!("Invalid mode. Use 'horizontal' or 'vertical'."),
        };

        // Get output or prompt for one
        let output = get_file_arg(FileType::Image, &self.output)?;

        // Save the image based on the output file extension and alpha channel
        let has_alpha = has_alpha_channel(&images);
        match output.extension().and_then(|ext| ext.to_str()) {
            Some("png") | Some("webp") if has_alpha => {
                // Save with alpha channel for PNG and WEBP
                result.save(&output)?;
            }
            _ => {
                // Convert to RGB for formats that don't support alpha (e.g., JPEG)
                let rgb_image = result.to_rgb8();
                DynamicImage::ImageRgb8(rgb_image).save(&output)?;
            }
        }

        Ok(())
    }
}

fn has_alpha_channel(images: &[DynamicImage]) -> bool {
    images
        .iter()
        .any(|img| matches!(img, DynamicImage::ImageRgba8(_)))
}

pub fn combine_images_horizontal(images: &[DynamicImage]) -> DynamicImage {
    let total_width = images.iter().map(|img| img.dimensions().0).sum::<u32>();
    let max_height = images.iter().map(|img| img.dimensions().1).max().unwrap();

    // Check if any image has an alpha channel
    let has_alpha = has_alpha_channel(images);

    if has_alpha {
        // Use RgbaImage if any image has an alpha channel
        let mut new_img = RgbaImage::new(total_width, max_height);
        let mut x_offset = 0;

        for img in images {
            let rgba_img = img.to_rgba8(); // Ensure the image is in RGBA format
            imageops::overlay(&mut new_img, &rgba_img, x_offset as i64, 0);
            x_offset += img.dimensions().0;
        }

        DynamicImage::ImageRgba8(new_img)
    } else {
        // Use RgbImage if no image has an alpha channel
        let mut new_img = RgbImage::new(total_width, max_height);
        let mut x_offset = 0;

        for img in images {
            let rgb_img = img.to_rgb8(); // Ensure the image is in RGB format
            imageops::overlay(&mut new_img, &rgb_img, x_offset as i64, 0);
            x_offset += img.dimensions().0;
        }

        DynamicImage::ImageRgb8(new_img)
    }
}

pub fn combine_images_vertical(images: &[DynamicImage]) -> DynamicImage {
    let max_width = images.iter().map(|img| img.dimensions().0).max().unwrap();
    let total_height = images.iter().map(|img| img.dimensions().1).sum::<u32>();

    // Check if any image has an alpha channel
    let has_alpha = has_alpha_channel(images);

    if has_alpha {
        // Use RgbaImage if any image has an alpha channel
        let mut new_img = RgbaImage::new(max_width, total_height);
        let mut y_offset = 0;

        for img in images {
            let rgba_img = img.to_rgba8(); // Ensure the image is in RGBA format
            imageops::overlay(&mut new_img, &rgba_img, 0, y_offset as i64);
            y_offset += img.dimensions().1;
        }

        DynamicImage::ImageRgba8(new_img)
    } else {
        // Use RgbImage if no image has an alpha channel
        let mut new_img = RgbImage::new(max_width, total_height);
        let mut y_offset = 0;

        for img in images {
            let rgb_img = img.to_rgb8(); // Ensure the image is in RGB format
            imageops::overlay(&mut new_img, &rgb_img, 0, y_offset as i64);
            y_offset += img.dimensions().1;
        }

        DynamicImage::ImageRgb8(new_img)
    }
}

#[derive(Args)]
pub struct FilterArgs {
    #[arg(short, long)]
    pub input: PathBuf,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "Filter type: grayscale, brightness, contrast, blur"
    )]
    pub filter: String,
    #[arg(long, help = "Filter intensity (for brightness, contrast, blur)")]
    pub intensity: Option<f32>,
}

impl FilterArgs {
    pub fn execute(&self) -> Result<()> {
        let img = image::open(&self.input)?;
        let result = match self.filter.as_str() {
            "grayscale" => apply_grayscale(&img),
            "brightness" => apply_brightness(&img, self.intensity.unwrap_or(1.0)),
            "contrast" => apply_contrast(&img, self.intensity.unwrap_or(1.0)),
            "blur" => apply_blur(&img, self.intensity.unwrap_or(1.0)),
            _ => anyhow::bail!("Invalid filter type."),
        };

        // Get output or prompt for one
        let output = get_file_arg(FileType::Image, &self.output)?;
        result.save(&output)?;
        Ok(())
    }
}

pub fn apply_grayscale(img: &DynamicImage) -> DynamicImage {
    img.grayscale()
}

pub fn apply_brightness(img: &DynamicImage, factor: f32) -> DynamicImage {
    let mut img = img.to_rgba8();
    for pixel in img.pixels_mut() {
        for channel in pixel.0.iter_mut().take(3) {
            *channel = (*channel as f32 * factor).min(255.0) as u8;
        }
    }
    DynamicImage::ImageRgba8(img)
}

pub fn apply_contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    let mut img = img.to_rgba8();
    for pixel in img.pixels_mut() {
        for channel in pixel.0.iter_mut().take(3) {
            *channel = ((*channel as f32 - 128.0) * factor + 128.0)
                .min(255.0)
                .max(0.0) as u8;
        }
    }
    DynamicImage::ImageRgba8(img)
}

pub fn apply_blur(img: &DynamicImage, sigma: f32) -> DynamicImage {
    let result = imageops::blur(img, sigma);
    DynamicImage::ImageRgba8(result)
}

#[derive(Args)]
pub struct ReshapeArgs {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(long, help = "Shape type: circle, square, rounded")]
    shape: String,
    #[arg(long, help = "Border radius for rounded shape")]
    radius: Option<u32>,
}

impl ReshapeArgs {
    fn execute(&self) -> Result<()> {
        let img = image::open(&self.input)?;
        let result = match self.shape.as_str() {
            "circle" => reshape_circle(&img),
            "square" => reshape_square(&img),
            "rounded" => reshape_rounded(&img, self.radius.unwrap_or(30)),
            _ => anyhow::bail!("Invalid shape type."),
        };

        // Get output or prompt for one
        let output = get_file_arg(FileType::Image, &self.output)?;
        result.save(&output)?;
        Ok(())
    }
}

pub fn reshape_circle(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let size = width.min(height);
    let mut mask = RgbaImage::new(size, size);
    let center = (size as f32 / 2.0, size as f32 / 2.0);
    let radius = size as f32 / 2.0;

    for x in 0..size {
        for y in 0..size {
            let dx = x as f32 - center.0;
            let dy = y as f32 - center.1;
            if dx * dx + dy * dy <= radius * radius {
                mask.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
    }

    let cropped =
        imageops::crop_imm(img, (width - size) / 2, (height - size) / 2, size, size).to_image();
    imageops::overlay(&mut mask, &DynamicImage::ImageRgba8(cropped), 0, 0);
    DynamicImage::ImageRgba8(mask)
}

pub fn reshape_square(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let size = width.min(height);
    let cropped =
        imageops::crop_imm(img, (width - size) / 2, (height - size) / 2, size, size).to_image();
    DynamicImage::ImageRgba8(cropped)
}

pub fn reshape_rounded(img: &DynamicImage, radius: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut mask = RgbaImage::new(width, height);
    let radius = radius as f32;

    for x in 0..width {
        for y in 0..height {
            let corners = [
                (x as f32, y as f32),
                (x as f32, (height - y - 1) as f32),
                ((width - x - 1) as f32, y as f32),
                ((width - x - 1) as f32, (height - y - 1) as f32),
            ];

            if corners
                .iter()
                .any(|&(cx, cy)| (cx - radius).powi(2) + (cy - radius).powi(2) > radius.powi(2))
            {
                mask.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
            } else {
                mask.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
    }

    let mut result = img.to_rgba8();
    imageops::overlay(&mut result, &DynamicImage::ImageRgba8(mask), 0, 0);
    DynamicImage::ImageRgba8(result)
}

#[derive(Args)]
pub struct CreateVideoArgs {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(long, help = "Duration of the video in seconds")]
    duration: u32,
}

impl CreateVideoArgs {
    fn execute(&self) -> Result<()> {
        let img = image::open(&self.input)?;

        // Get output or prompt for one
        let output = get_file_arg(FileType::Image, &self.output)?;
        create_video_from_image(&img, &output, self.duration)
    }
}

pub fn create_video_from_image(img: &DynamicImage, output: &PathBuf, duration: u32) -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let frame_path = temp_dir.path().join("frame.png");
    img.save(&frame_path)?;

    let args = [
        "-loop",
        "1",
        "-i",
        frame_path.to_str().unwrap(),
        "-c:v",
        "libx264",
        "-t",
        &duration.to_string(),
        "-pix_fmt",
        "yuv420p",
        "-y",
        output.to_str().unwrap(),
    ];

    crate::utils::run_ffmpeg_command(&args)
}

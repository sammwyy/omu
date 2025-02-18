use omu::image::{FilterArgs, OverlayArgs};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_overlay_images() {
    // Test overlaying one image on top of another.
    let temp_dir = tempdir().unwrap();
    let base = Path::new("samples/source.png");
    let overlay = Path::new("samples/source.webp");
    let output = temp_dir.path().join("overlay.png");

    // Perform the image overlay.
    let args = OverlayArgs {
        input: base.to_path_buf(),
        overlay: overlay.to_path_buf(),
        output: output.clone(),
        x: 50,
        y: 50,
    };
    args.execute().unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid PNG.
    let png_magic = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // Magic bytes of PNG.
    assert!(omu::utils::verify_magic_bytes(&output, png_magic).unwrap());
}

#[test]
fn test_apply_grayscale_filter() {
    // Test applying a grayscale filter to an image.
    let temp_dir = tempdir().unwrap();
    let input = Path::new("samples/source.png");
    let output = temp_dir.path().join("grayscale.png");

    // Perform the grayscale filter application.
    let args = FilterArgs {
        input: input.to_path_buf(),
        output: output.clone(),
        filter: "grayscale".to_string(),
        intensity: None,
    };
    args.execute().unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid PNG.
    let png_magic = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // Magic bytes of PNG.
    assert!(omu::utils::verify_magic_bytes(&output, png_magic).unwrap());
}

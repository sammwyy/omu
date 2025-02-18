use omu::convert::convert_file;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_convert_mp3_to_wav() {
    // Test converting an MP3 file to WAV format.
    let temp_dir = tempdir().unwrap();
    let input = Path::new("samples/source.mp3");
    let output = temp_dir.path().join("output.wav");

    // Perform the conversion.
    convert_file(input, &output, None).unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid WAV.
    let wav_magic = b"RIFF"; // First 4 bytes of a WAV file.
    assert!(omu::utils::verify_magic_bytes(&output, wav_magic).unwrap());
}

#[test]
fn test_convert_png_to_jpg() {
    // Test converting a PNG file to JPEG format.
    let temp_dir = tempdir().unwrap();
    let input = Path::new("samples/source.png");
    let output = temp_dir.path().join("output.jpg");

    // Perform the conversion.
    convert_file(input, &output, None).unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid JPEG.
    let jpeg_magic = &[0xFF, 0xD8, 0xFF]; // First 3 bytes of a JPEG file.
    assert!(omu::utils::verify_magic_bytes(&output, jpeg_magic).unwrap());
}

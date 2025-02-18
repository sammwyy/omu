use omu::video::{ExtractAudioArgs, TrimArgs};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_extract_audio_from_video() {
    // Test extracting audio from a video file.
    let temp_dir = tempdir().unwrap();
    let input = Path::new("samples/source.mp4");
    let output = temp_dir.path().join("audio.wav");

    // Perform the audio extraction.
    let args = ExtractAudioArgs {
        input: input.to_path_buf(),
        output: Some(output.clone()),
    };
    args.execute().unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid WAV.
    let wav_magic = b"RIFF"; // First 4 bytes of a WAV file.
    assert!(omu::utils::verify_magic_bytes(&output, wav_magic).unwrap());
}

#[test]
fn test_trim_video() {
    // Test trimming a video file.
    let temp_dir = tempdir().unwrap();
    let input = Path::new("samples/source.mp4");
    let output = temp_dir.path().join("trimmed.mp4");

    // Perform the video trimming.
    let args = TrimArgs {
        input: input.to_path_buf(),
        output: Some(output.clone()),
        start: Some("00:00:01".to_string()),
        end: Some("00:00:03".to_string()),
    };
    args.execute().unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid MP4.
    let mp4_magic = &[0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70]; // Magic bytes of MP4.
    assert!(omu::utils::verify_magic_bytes(&output, mp4_magic).unwrap());
}

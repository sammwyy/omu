use assert_cmd::Command;
use omu::utils::create_temp_file;

#[test]
fn test_cli_convert_mp3_to_wav() {
    // Test the CLI command for converting MP3 to WAV.
    let output = create_temp_file(&"wav".to_string());

    let mut cmd = Command::cargo_bin("omu").unwrap();
    cmd.args(&[
        "convert",
        "-i",
        "samples/source.mp3",
        "-o",
        output.to_str().unwrap(),
    ])
    .assert()
    .success();

    // Verify that the output file exists.
    assert!(output.exists());

    // Verify the magic bytes to ensure the file is a valid WAV.
    let wav_magic = b"RIFF"; // First 4 bytes of a WAV file.
    assert!(omu::utils::verify_magic_bytes(output.as_path(), wav_magic).unwrap());
}

#[test]
fn test_cli_extract_audio_from_video() {
    // Test the CLI command for extracting audio from a video.
    let output = create_temp_file(&"wav".to_string());

    let mut cmd = Command::cargo_bin("omu").unwrap();
    cmd.args(&[
        "video",
        "extract-audio",
        "-i",
        "samples/source.mp4",
        "-o",
        output.to_str().unwrap(),
    ])
    .assert()
    .success();

    // Verify that the output file exists.
    assert!(output.exists());

    // Verify the magic bytes to ensure the file is a valid WAV.
    let wav_magic = b"RIFF"; // First 4 bytes of a WAV file.
    assert!(omu::utils::verify_magic_bytes(output.as_path(), wav_magic).unwrap());
}

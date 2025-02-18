use omu::audio::{CombineAudioArgs, VolumeArgs};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_combine_audios() {
    // Test combining two audio files into one.
    let temp_dir = tempdir().unwrap();
    let input1 = Path::new("samples/source.mp3");
    let input2 = Path::new("samples/source.wav");
    let output = temp_dir.path().join("combined.mp3");

    // Perform the audio combination.
    let args = CombineAudioArgs {
        inputs: vec![input1.to_path_buf(), input2.to_path_buf()],
        output: output.clone(),
    };
    args.execute().unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid MP3.
    let mp3_magic = b"ID3"; // First 3 bytes of an MP3 file.
    assert!(omu::utils::verify_magic_bytes(&output, mp3_magic).unwrap());
}

#[test]
fn test_change_audio_volume() {
    // Test changing the volume of an audio file.
    let temp_dir = tempdir().unwrap();
    let input = Path::new("samples/source.mp3");
    let output = temp_dir.path().join("louder.mp3");

    // Perform the volume change.
    let args = VolumeArgs {
        input: input.to_path_buf(),
        output: output.clone(),
        volume: 2.0,
    };
    args.execute().unwrap();

    // Verify that the output file exists and is not empty.
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);

    // Verify the magic bytes to ensure the file is a valid MP3.
    let mp3_magic = b"ID3"; // First 3 bytes of an MP3 file.
    assert!(omu::utils::verify_magic_bytes(&output, mp3_magic).unwrap());
}

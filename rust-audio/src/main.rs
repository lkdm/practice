use opus_pure::{Application, MAX_PACKET_BYTES, OggOpusWriter, OpusEncoder, OpusHead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Audio configuration
    let sample_rate = 48_000;
    let channels = 1;
    let frame_size = 960;

    // 960 samples at 48 kHz = 20 milliseconds.
    let seconds = 10;
    let frames = seconds * 50;

    // Create the Opus encoder.
    let mut encoder = OpusEncoder::new(sample_rate, channels, Application::Audio)?;

    encoder.bitrate_bps = 16_000;

    // Create OGG container.
    let head = OpusHead::for_encoder(&encoder, sample_rate as u32);

    let mut writer = OggOpusWriter::new(Vec::new(), head)?;

    // One 20 ms buffer of silence.
    let silence = vec![0.0f32; frame_size * channels];

    // Reusable output buffer for compressed packets.
    let mut packet = vec![0u8; MAX_PACKET_BYTES];

    // Generate audio frame by frame.
    for _ in 0..frames {
        let encoded_size = encoder.encode(&silence, frame_size, &mut packet)?;

        writer.write_packet(&packet[..encoded_size])?;
    }

    // Finalize the OGG stream.
    let ogg_data = writer.finish()?;

    // Save the file.
    std::fs::write("silence.opus", &ogg_data)?;

    println!("Generated {} bytes of audio", ogg_data.len());

    Ok(())
}

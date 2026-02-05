use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct AudioCapture {
    _stream: Stream,
}

impl AudioCapture {
    pub fn start(tx: mpsc::UnboundedSender<Vec<i16>>) -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();

        // Get default INPUT device (microphone)
        let device = host
            .default_input_device()
            .ok_or("No microphone found")?;

        println!("[Audio] Using device: {}", device.name()?);

        let config = device.default_input_config()?;
        println!("[Audio] Config: {:?}", config);

        // Build stream based on sample format
        let stream = match config.sample_format() {
            SampleFormat::F32 => Self::build_stream::<f32>(&device, &config.into(), tx)?,
            SampleFormat::I16 => Self::build_stream::<i16>(&device, &config.into(), tx)?,
            SampleFormat::U16 => Self::build_stream::<u16>(&device, &config.into(), tx)?,
            format => return Err(format!("Unsupported sample format: {:?}", format).into()),
        };

        stream.play()?;

        Ok(Self { _stream: stream })
    }

    fn build_stream<T>(
        device: &Device,
        config: &StreamConfig,
        tx: mpsc::UnboundedSender<Vec<i16>>,
    ) -> Result<Stream, Box<dyn std::error::Error>>
    where
        T: Sample + cpal::SizedSample,
        i16: From<T>,
    {
        let channels = config.channels as usize;
        let tx = Arc::new(tx);

        let stream = device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                // Convert to mono i16
                let mono: Vec<i16> = data
                    .chunks(channels)
                    .map(|frame| {
                        // Average channels to mono
                        let sum: i32 = frame.iter().map(|&s| i16::from(s) as i32).sum();
                        (sum / channels as i32) as i16
                    })
                    .collect();

                // Send in chunks of ~100ms (1600 samples at 16kHz)
                for chunk in mono.chunks(1600) {
                    let _ = tx.send(chunk.to_vec());
                }
            },
            move |err| {
                eprintln!("[Audio] Error: {}", err);
            },
            None,
        )?;

        Ok(stream)
    }
}

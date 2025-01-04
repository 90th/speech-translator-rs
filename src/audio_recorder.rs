use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleFormat, StreamConfig};
use hound::{WavWriter, WavSpec};
use std::{fs::File, sync::{Arc, Mutex}};
use anyhow::Result;
use dasp::sample::ToSample;

pub struct AudioRecorder {
    writer: Arc<Mutex<Option<WavWriter<File>>>>,
    stream: Option<cpal::Stream>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        AudioRecorder {
            writer: Arc::new(Mutex::new(None)),
            stream: None,
        }
    }

    pub fn start_recording(&mut self, output_file_path: &str) {
        let host = cpal::default_host();
        let device = host.default_input_device().expect("Failed to get input device");

        let config = device.default_input_config().expect("Failed to get default input configuration");
        let sample_format = config.sample_format();
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        let spec = WavSpec {
            channels: channels as u16,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let file = File::create(output_file_path).expect("Failed to create file");
        let writer = WavWriter::new(file, spec).expect("Failed to create WAV writer");
        *self.writer.lock().unwrap() = Some(writer);

        let stream_config = StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let writer_clone = Arc::clone(&self.writer);
        let stream = match sample_format {
            SampleFormat::F32 => self.build_input_stream::<f32>(&device, &stream_config, writer_clone),
            SampleFormat::I16 => self.build_input_stream::<i16>(&device, &stream_config, writer_clone),
            SampleFormat::U16 => self.build_input_stream::<u16>(&device, &stream_config, writer_clone),
            _ => panic!("Unsupported sample format"),
        }.expect("Failed to build input stream");

        stream.play().expect("Failed to start input stream");
        self.stream = Some(stream);
        println!("Recording started...");
    }

    fn build_input_stream<T>(&self, device: &cpal::Device, config: &StreamConfig, writer: Arc<Mutex<Option<WavWriter<File>>>>) -> Result<cpal::Stream, cpal::BuildStreamError>
    where
        T: cpal::Sample + cpal::SizedSample + ToSample<f32> + 'static,
    {
        device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                if let Some(writer) = &mut *writer.lock().unwrap() {
                    for &sample in data.iter() {
                        let sample: i16 = (sample.to_sample::<f32>() * i16::MAX as f32) as i16;
                        writer.write_sample(sample).expect("Failed to write sample");
                    }
                }
            },
            |err| {
                eprintln!("Error: {}", err);
            },
            None,
        )
    }

    pub fn stop_recording(&mut self) {
        if let Some(writer) = self.writer.lock().unwrap().take() {
            writer.finalize().expect("Failed to finalize WAV writer");
            println!("Recording stopped.");
        }

        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}
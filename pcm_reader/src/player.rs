// funções auxiliares e os krl
#![allow(dead_code)]

use std::{fs::File, io::Read};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::platform::{Host, Device};
use std::time::Duration;
use cpal::{StreamConfig};
use std::io::Error;
use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct System {
	pub host: Host,
	pub device: Device,
	pub config: StreamConfig,
}

impl System {
	pub fn new() -> Self {
	    let host = cpal::default_host();
	    let device = host.default_output_device().expect("No output device available");
	    let config = device.default_output_config().unwrap().config();

	    Self {
	    	host, 
	    	device,
	    	config,
	    }
	}

	pub fn move_timer(&self) { }
	pub fn change_audio(&self) { }

	pub fn run(&self, audio: Audio) {
		let sample_rate = self.config.sample_rate.0 as f32;
		let channels = self.config.channels as usize;
		let sample_rate_factor = sample_rate / audio.sample_rate;

		let is_paused = Arc::new(Mutex::new(false));
		let move_index: Arc<Mutex<Option<usize>>> = 
		Arc::new(Mutex::new(None));

		{
			let is_paused = Arc::clone(&is_paused);
			let move_index = Arc::clone(&move_index);
			thread::spawn(move || {
				loop {
					let mut input = String::new();
					std::io::stdin().read_line(&mut input).unwrap();

					if input.trim() == "p" {
						let mut paused = is_paused.lock().unwrap();
						*paused = !*paused;
					}

					if input.trim().chars().all(|c| c.is_digit(10)) {
						let mut moved = move_index.lock().unwrap();

						if let Ok(number) = input.trim().parse::<usize>() {
							let k = number * (audio.sample_rate as usize);

							*moved = Some(k);
						}
					}
				}
			});
		}

		let mut pcm_index = 0;

	    let stream = self.device.build_output_stream(
	        &self.config,
	        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
	        	let paused = is_paused.lock().unwrap();

	        	if *paused {
	        		for frame in data.chunks_mut(channels) {
	        			for sample_out in frame.iter_mut() {
	        				*sample_out = 0.0;
	        			}
	        		}

	        		return;
	        	}

	        	let mut moved = move_index.lock().unwrap();

	        	if let Some(number) = *moved {
	        		pcm_index = number;
	        		*moved = None;
	        	}

	            for frame in data.chunks_mut(channels) {
	                let adjusted_index = (pcm_index as f32 * sample_rate_factor)
	                                     .round() as usize;

	                if adjusted_index + 1 < audio.data.len() {
	                    let sample = i16::from_le_bytes
	                    ([audio.data[adjusted_index], audio.data[adjusted_index + 1]]) as f32;
	                    let normalized_sample = sample / i16::MAX as f32;

	                    for sample_out in frame.iter_mut() {
	                        *sample_out = normalized_sample;
	                    }

	                    pcm_index += 2;
	                } else {
	                    for sample_out in frame.iter_mut() {
	                        *sample_out = 0.0;
	                    }
	                }
	            }
	        },
	        |err| eprintln!("Error: {:?}", err),
	        None,
	    ).unwrap();

	    stream.play().unwrap();
		std::thread::sleep(Duration::from_secs(audio.length));
	}
}

pub struct Audio {
	pub file_path: String,
	pub sample_rate: f32,
	pub length: u64,
	pub data: Vec<u8>,
}

impl Audio {
	pub fn new(file_path: &str) -> Result<Self, Error> {
		let mut file = File::open(file_path)?;

		let mut pcm_data: Vec<u8> = Vec::new();
		let sample_rate = 22050.0;

		match file.read_to_end(&mut pcm_data) {
			Ok(_) => Ok(
				Audio {
					file_path: String::from(file_path),
					sample_rate,
					length: max((((pcm_data.len()) as f32 / 4.0) / sample_rate) as u64, 1),
					data: pcm_data,
				}
			),
			Err(err) => Err(err),
		}	
	}
}

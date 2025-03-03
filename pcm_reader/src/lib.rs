// funções auxiliares e os krl

use std::{fs::File, io::Read};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::platform::{Host, Device};
use cpal::{StreamConfig};
use std::io::Error;
use std::cmp::max;

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
					length: max((((pcm_data.len() / 4) as f32) / sample_rate) as u64, 1),
					data: pcm_data,
				}
			),
			Err(err) => Err(err),
		}	
	}
}

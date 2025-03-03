/*
    Código "padrão" de todos os projetos.
    Explicação linha por linha + ajustes:

    DeviceTrait, HostTrait, StreamTrait -> traits essencias para
    trabalhar com áudio.

    Host -> Resultado de cpal::default_host, e é o sistema de 
    aúdio padrão (ALSA, PulseAudio, etc)

    Device -> Resultado de host.default_output_device, e é o
    dispositivo de saída padrão (fones, caixas de som, etc)

    Config -> Resultado de device.default_output_config e é
    a configuração padrão (sample rate, canais, etc)

    sample_rate -> Resultado de config.sample_rate, e é o sample_rate.
    channels -> Resultado de config.channels e são os canais.

    step -> incremento para gerar um lá 440hz.

    stream -> cria o stream de áudio
    build_output_stream -> cria um fluxo de audio que envia dados para o dispositivo
    data -> array de samples
    data.chunks_mut(channels) -> divide o buffer em numero de canais
    frame -> elemento de data, que é divido em 1 (mono) ou 2 (estéreo)
    sample -> amplitude da onda em determinado instante
    sample_out -> coloca o sample em cada saída (L e R se for estéreo)

    stream.play -> toca o audio


*/

extern crate pcm_reader;
use pcm_reader::*;
use cpal::traits::{DeviceTrait, StreamTrait};
use std::time::Duration;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("### Usage: cargo run -- filepath ###");
    }

    let system = System::new();

    let sample_rate = system.config.sample_rate.0 as f32; // Taxa de amostragem do dispositivo
    let channels = system.config.channels as usize;

    let audio = Audio::new(args[1].as_ref())
                   .unwrap_or_else(|err| {
                    eprintln!("Problem reading file: {}", err);
                    process::exit(1);
                   });

    let sample_rate_factor = sample_rate / audio.sample_rate;

    let mut pcm_index = 0;

    let stream = system.device.build_output_stream(
        &system.config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
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



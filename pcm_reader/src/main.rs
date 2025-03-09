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

#![allow(dead_code, unused_imports)]

mod player;
mod gui;

use player::*;
use gui::*;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("### Usage: cargo run -- filepath ###");
    }

    let system = System::new();

    let audio = Audio::new(args[1].as_ref())
                   .unwrap_or_else(|err| {
                    eprintln!("Problem reading file: {}", err);
                    process::exit(1);
                   });

    system.run(audio);
}



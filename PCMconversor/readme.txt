# Projeto de Conversão de MP3 para PCM

Este projeto é uma ferramenta que converte arquivos de áudio MP3 em arquivos de áudio PCM. Os arquivos PCM gerados são salvos na pasta `cache`.
 O programa lê a pasta `musica`, lista as músicas disponíveis e permite ao usuário escolher qual música deseja converter para PCM.

### Dependências

Para compilar e rodar este projeto, você precisará das seguintes dependências:

- **GCC (GNU Compiler Collection)**: Necessário para compilar os arquivos de código em C. 
Pode ser instalado no Windows através do MinGW, no Linux com o `build-essential` e no macOS através das ferramentas de linha de comando do Xcode.

### Instalação das Dependências

#### 1. Instalar o **GCC** (Compilador C)

##### No Windows:

1. **Instalar o MinGW** (Minimalist GNU for Windows):
   - Acesse o site do MinGW: [MinGW SourceForge](https://osdn.net/projects/mingw/releases/).
   - Baixe o instalador **mingw-get-setup.exe** e execute-o.
   - Durante a instalação, selecione o pacote **mingw32-gcc-g++** (compilador C/C++).
   - Após a instalação, adicione o caminho do **MinGW** ao **PATH** do seu sistema:
     - Clique com o botão direito em "Este Computador" e selecione **Propriedades**.
     - Clique em **Configurações Avançadas do Sistema** > **Variáveis de Ambiente**.
     - Em **Variáveis do Sistema**, localize e edite a variável **Path** e adicione o caminho do diretório `bin` do MinGW, como por exemplo `C:\MinGW\bin`.
   - Verifique a instalação do **GCC** no terminal (Prompt de Comando ou PowerShell):
     ```bash
     gcc --version
     ```
     Se estiver corretamente instalado, ele retornará a versão do GCC.

##### No Linux (Ubuntu/Debian):

1. **Instalar o GCC**:
   Abra o terminal e instale o **GCC** usando o seguinte comando:
   ```bash
   sudo apt update
   sudo apt install build-essential 



---------------Aqui está o manual de instruções para compilar e integrar o código C com o código Rust.--------------------




Manual de Compilação e Integração de C com Rust

1. Preparação do Ambiente no Linux:

1. Instalar o compilador C: Certifique-se de que o compilador gcc e o make estão instalados. Caso contrário, instale-os com o seguinte comando:

sudo apt update
sudo apt install gcc make



2. Arquivos C:

Envie os seguintes arquivos C para o projeto:

main.c:

#include "mp3_decoder.h"

// Implementação das funções C
void listar_musicas() {
    // Código para listar músicas (sem alteração)
}

void escolher_musica(char *musica_escolhida, size_t tamanho) {
    listar_musicas();
    printf("\nDigite o nome da música (com extensão .mp3): ");
    scanf("%255s", musica_escolhida);
}

int converter_mp3_para_pcm(const char *musica_escolhida) {
    // Lógica para converter MP3 para PCM (sem alteração)
    return 0;  // Retorna 0 em caso de sucesso
}

mp3_decoder.h:

#ifndef MP3_DECODER_H
#define MP3_DECODER_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>

// Função para listar arquivos MP3
void listar_musicas();

// Função para escolher a música
void escolher_musica(char *musica_escolhida, size_t tamanho);

// Função para converter MP3 para PCM
int converter_mp3_para_pcm(const char *musica_escolhida);

#endif

3. Compilação do Código C:

Para compilar o código C em uma biblioteca compartilhada no Linux, use o seguinte comando:

gcc -shared -o libmp3decoder.so -fPIC main.c

gcc: O compilador C.

-shared: Gera uma biblioteca compartilhada.

-o libmp3decoder.so: Nome do arquivo da biblioteca.

-fPIC: Gera código de posição independente (necessário para bibliotecas compartilhadas).


Esse comando criará a biblioteca libmp3decoder.so no diretório atual.

4. Integrando a Biblioteca C com o Projeto Rust:

Adicionar Dependência no Cargo.toml:

No arquivo Cargo.toml do projeto Rust, adicione a dependência libloading para permitir o carregamento dinâmico da biblioteca C:

[dependencies]
libloading = "0.7"

Código Rust:

Crie o código Rust que fará uso da biblioteca C. Este código chama as funções escolher_musica e converter_mp3_para_pcm da biblioteca C compilada.

extern crate libloading;

use libloading::{Library, Symbol};
use std::ffi::CString;
use std::ptr;

extern "C" {
    fn escolher_musica(musica_escolhida: *mut i8, tamanho: usize);
    fn converter_mp3_para_pcm(musica_escolhida: *const i8) -> i32;
}

fn main() {
    // Carregar a biblioteca C compartilhada
    let lib = Library::new("./libmp3decoder.so").expect("Não foi possível carregar a biblioteca");

    unsafe {
        // Criar uma string C para armazenar o nome da música escolhida
        let musica_escolhida = CString::new("").expect("Falha ao criar CString");
        let musica_ptr = musica_escolhida.as_ptr() as *mut i8;

        // Chamar a função C escolher_musica para que o usuário escolha uma música
        escolher_musica(musica_ptr, 256);

        // A partir da escolha, a música será armazenada em musica_escolhida
        // Aqui, você pode obter a música escolhida convertendo de volta para uma string Rust
        let musica_escolhida_rust = musica_escolhida.to_str().expect("Erro ao converter música escolhida para string");
        println!("Música escolhida: {}", musica_escolhida_rust);

        // Agora, chamamos a função C para converter a música em PCM
        let musica_para_converter = CString::new(musica_escolhida_rust).expect("Falha ao criar CString");
        let musica_ptr = musica_para_converter.as_ptr();

        // Chamar a função de conversão
        let resultado = converter_mp3_para_pcm(musica_ptr);

        if resultado == 0 {
            println!("Conversão para PCM bem-sucedida!");
        } else {
            println!("Erro na conversão para PCM.");
        }
    }
}

5. Executando o Projeto:

1. Estrutura do projeto:



A estrutura de diretórios do projeto deverá ser assim:

projeto_rust/
├── src/
│   └── main.rs
├── libmp3decoder.so  (compilado do código C)
└── Cargo.toml

2. Compilação e Execução:



No terminal, no diretório do projeto Rust, execute os seguintes comandos:

cargo build
cargo run

6. Considerações Finais:

A biblioteca libmp3decoder.so deve estar no mesmo diretório que o código Rust ou em um caminho configurado corretamente.

Certifique-se de que as funções C estão corretamente declaradas no arquivo .h para que possam ser chamadas no Rust com extern "C".

O código Rust usa a biblioteca C carregada dinamicamente para realizar as funções de escolha de música e conversão para PCM.


Com isso, o código C será integrado corretamente ao projeto Rust, e as funções de escolha de músicas e conversão para PCM estarão disponíveis para uso no código Rust.

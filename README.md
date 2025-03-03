# first-project
primeiro projeto poggers loggers

Antes de usar certifique de ter as versões mais recentes de **rustc**, **cargo** e **gcc** instaladas. Para isso:

# Rust

### Linux e macOS

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Após a instalação, adicione Rust ao seu PATH (caso ainda não esteja):

```sh
source $HOME/.cargo/env
```

Para verificar se rustc e cargo estão instalados use:

```sh
rustc --version
cargo --version
```

### Windows

Baixe no próprio site do Rust: https://rustup.rs/

# C

### Linux

A maioria já vem com **gcc** instalado, mas tá aí ne pae:

```sh
# Debian/Ubuntu
sudo apt update && sudo apt install build-essential

# Arch Linux
sudo pacman -S base-devel

# Fedora
sudo dnf install gcc

# openSUSE
sudo zypper install gcc
```

Para verificar a instalação:

```sh
gcc --version
```

### Windows e macOS

Não sei.

# Como usar

Certifique de estar dentro da pasta [pcm_reader](pcm_reader/) e use o seguinte código na linha de comando:

```sh
cargo run -- filepath
```

Onde **filepath** é o caminho relativo ao arquivo .pcm que quer que seja lido (já coloquei alguns em [assets](pcm_reader/assets)).

Agora é só curtir o batidão.


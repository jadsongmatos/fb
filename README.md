# Linux Framebuffer Capture and Manipulation

## Overview

Este projeto demonstra como capturar e manipular frames do framebuffer do Linux em tempo real. Utilizamos a biblioteca `linuxfb` em Rust para acessar e modificar os frames diretamente do `/dev/fb0`.

## Features

- Listagem de dispositivos framebuffer disponíveis.
- Acesso e manipulação de frames em tempo real.
- Configuração de cores RGB para manipulação dos pixels.

## Requirements

- Rust
- Biblioteca `linuxfb`
- Sistema operacional Linux com acesso ao framebuffer (`/dev/fb0`)

## Installation

```bash
cargo build
```

## Usage

### Captura do Framebuffer

Utilize o comando `ffmpeg` para capturar frames do framebuffer:

```bash
ffmpeg -f fbdev -framerate 30 -i /dev/fb0 -f sdl "Framebuffer"
```

### Execução do Script Rust

1. Execute o script Rust para listar dispositivos e acessar o framebuffer:
```bash
sudo ./target/debug/fb
```
2. O script irá exibir informações sobre o dispositivo e manipular os pixels em tempo real.

## Contributing

Contribuições são bem-vindas! Se você tem melhorias ou correções, sinta-se à vontade para fazer um fork do repositório e abrir um pull request.
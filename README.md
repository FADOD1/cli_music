# CLI Music Player 🎵

## Sobre
`cli_music` é um reprodutor de música minimalista para terminal, desenvolvido em Rust. Ele permite navegar e reproduzir arquivos MP3 armazenados no diretório `music/` com controles simples diretamente pelo teclado.

## Funcionalidades
- Listagem automática de arquivos `.mp3` no diretório `music/`.
- Controle de reprodução (Play/Pause).
- Navegação entre músicas com as setas do teclado.
- Ajuste de volume (+/-).
- Interface interativa no terminal com informações sobre a música atual.

## Controles
- `↑ / ↓` - Navegar entre as músicas.
- `Enter` - Selecionar e tocar a música.
- `Espaço` - Pausar/Retomar a reprodução.
- `+ / -` - Aumentar/Diminuir o volume.
- `q / Esc` - Sair do player.

## Instalação
### Pré-requisitos
- [Rust e Cargo](https://www.rust-lang.org/tools/install) instalados.

### Clonando o repositório e compilando
```sh
# Clonar o repositório
git clone https://github.com/FADOD1/cli_music
cd cli_music

# Compilar o projeto
cargo build --release

# Executar o player
./target/release/cli_music
```

## Como usar
1. Certifique-se de que o diretório `music/` existe na raiz do projeto e contém arquivos MP3.
2. Execute o player via terminal: `./target/release/cli_music`.
3. Utilize os controles mencionados acima para gerenciar a reprodução.

## Dependências
Este projeto utiliza as seguintes crates:
- `crossterm` - Para manipulação do terminal.
- `rodio` - Para reprodução de áudio.

## 📥 Download do Binário

Agora você pode usar o `CLI Music Player` sem precisar compilar! 🎵  

Baixe a versão mais recente:  
- **Linux/macOS:** [Baixar cli_music](https://github.com/FADOD1/cli_music/releases/latest/download/cli_music)  

### **Como executar**
#### **Linux/macOS**
```sh
chmod +x cli_music
./cli_music


## Licença
Este projeto é licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para mais detalhes.


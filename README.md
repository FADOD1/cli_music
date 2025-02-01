# CLI Music Player 🎵



https://github.com/user-attachments/assets/653a83e3-e8b1-4c9e-bc5a-6277fe721cd0



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




## 📥 Download do Binário para Linux

Caso prefira você pode usar o `CLI Music Player` em plataformas x86_64 Linux sem precisar compilar! 🎵  

Baixe a versão mais recente:  
- **Linux:** [Baixar cli_music](https://github.com/FADOD1/cli_music/releases/tag/v2.0.0)

Crie uma pasta que servirá para colocar o cli_music, por exemplo: `cli_music`
adicione o binario a essa pasta e execute-o , a pasta music e criada automaticamente e nela que deve
se adicionadas as suas musicas.

Atenção: caso não crie a pasta `music` automaticamente ela deve ser criada!!











## Passos para criar um alias para executar o cli_music sem a necessidade de entrar no diretorio e executar manualmente:

Abra o arquivo de configuração do seu shell (dependendo do shell que você está usando):

Para Bash:

```sh
nano ~/.bashrc
```

Para Zsh:

```sh
nano ~/.zshrc

```

Crie o alias para o binário, informando o caminho do binário do cli_music:

```sh
alias climusic='/caminho/para/o/binario/nome_do_binario'
```

Salve e saia do editor (`CTRL + X`, depois `Y` para confirmar e Enter para salvar).

- Recarregue o arquivo de configuração para que as mudanças entrem em vigor:

Para Bash:

```sh
source ~/.bashrc
```

Para Zsh:

```sh
source ~/.zshrc
```

Agora, sempre que você digitar `climusic` no terminal, o cli_music será executado.



## **Como executar sem o alias**
 **Linux**
```sh
chmod +x cli_music
./cli_music
```










## Licença
Este projeto é licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para mais detalhes.


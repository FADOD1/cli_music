use crossterm::{
    cursor::MoveTo,
    event::{self, KeyCode, KeyEvent},
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::{self, File},
    io::{stdout, Write},
    path::Path,
    time::{Duration, Instant},
};

struct Song {
    name: String,
    path: String,
}

impl Song {
    fn new(name: &str, path: &str) -> Self {
        Song {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

pub fn music_player() {
    let music_dir = Path::new("music");
    if !music_dir.exists() {
        fs::create_dir_all(music_dir).unwrap();
    }

    let mut songs = Vec::new();
    if let Ok(entries) = fs::read_dir(music_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().map(|ext| ext == "mp3").unwrap_or(false) {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                songs.push(Song::new(&name, path.to_str().unwrap()));
            }
        }
    }

    if songs.is_empty() {
        println!("No songs found in 'music' directory.");
        return;
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut current_song = 0;
    let mut volume = 0.5;
    let mut index = 0;
    let mut total_elapsed = Duration::default();
    let mut play_start: Option<Instant> = None;

    // Initialize playback state
    let mut playing = true;
    let file = File::open(&songs[0].path).unwrap();
    sink.append(Decoder::new(file).unwrap());
    play_start = Some(Instant::now());
    sink.set_volume(volume);

    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    loop {
        queue!(stdout, Clear(ClearType::All)).unwrap();

        let width = 50;
        let line = "-".repeat(width);
        let title = "🎹 CLI MUSIC PLAYER 🎵";
        let song_list_start = 9;
        let status_line = song_list_start + songs.len() as u16 + 2;

        // Header section
        queue!(
            stdout,
            MoveTo(0, 0),
            Print(format!("{}\n", line)),
            MoveTo(0, 1),
            Print(format!("{:^50}\n", title)),
            MoveTo(0, 2),
            Print(format!("{}\n", line)),
            MoveTo(0, 4),
            Print("Controls:"),
            MoveTo(0, 5),
            Print("↑/↓: Navigate  Space: Play/Pause"),
            MoveTo(0, 6),
            Print("Enter: Select  +/-: Volume  q: Quit"),
            MoveTo(0, 7),
            Print(format!("{}\n", line)),
        )
        .unwrap();

        // Song list
        for (i, song) in songs.iter().enumerate() {
            let y_pos = song_list_start + i as u16;
            queue!(stdout, MoveTo(0, y_pos)).unwrap();
            let display = if i == index {
                format!("▶ {}", song.name)
            } else {
                song.name.clone()
            };
            if i == index {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(display),
                    ResetColor
                )
                .unwrap();
            } else {
                queue!(stdout, Print(display)).unwrap();
            }
        }

        // Playback info (positioned below song list)
        let status = if playing { "▶ Playing" } else { "⏸ Paused" };
        let elapsed = if let Some(start) = play_start {
            total_elapsed + start.elapsed()
        } else {
            total_elapsed
        };
        let elapsed = format!("{:.0?}", elapsed);
        let filled = (volume * 10.0).round() as usize;
        let volume_bar = format!("Vol: {}{}", "▣".repeat(filled), "━".repeat(10 - filled));

        queue!(
            stdout,
            MoveTo(0, status_line),
            Print(format!("{} {}", status, elapsed)),
            MoveTo(0, status_line + 1),
            Print(volume_bar),
        )
        .unwrap();

        stdout.flush().unwrap();

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Up if index > 0 => index -= 1,
                    KeyCode::Down if index < songs.len() - 1 => index += 1,

                    KeyCode::Char(' ') => {
                        playing = !playing;
                        if playing {
                            play_start = Some(Instant::now());
                            sink.play();
                        } else {
                            if let Some(start) = play_start.take() {
                                total_elapsed += start.elapsed();
                            }
                            sink.pause();
                        }
                    }

                    KeyCode::Char('+') if volume < 1.0 => {
                        volume = (volume + 0.1).min(1.0);
                        sink.set_volume(volume);
                    }

                    KeyCode::Char('-') if volume > 0.0 => {
                        volume = (volume - 0.1).max(0.0);
                        sink.set_volume(volume);
                    }

                    KeyCode::Enter => {
                        if current_song != index || sink.empty() {
                            sink.stop();
                            let file = File::open(&songs[index].path).unwrap();
                            sink.append(Decoder::new(file).unwrap());
                            current_song = index;
                            playing = true;
                            play_start = Some(Instant::now());
                            total_elapsed = Duration::default();
                        }
                    }

                    KeyCode::Esc | KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    sink.stop();
    disable_raw_mode().unwrap();
}
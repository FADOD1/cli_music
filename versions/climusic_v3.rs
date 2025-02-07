// v3 development version

use crossterm::{
    cursor::MoveTo,
    event::{self, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size}
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{stdout, Write},
    path::Path,
    thread,
    time::{Duration, Instant},
};
use ctrlc;

#[derive(Clone)]
struct Song {
    name: String,
    path: String,
    duration: Duration,
}

impl Song {
    fn new(name: String, path: String, duration: Duration) -> Self {
        Song { name, path, duration }
    }
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", mins, secs)
}

fn draw_box(x: u16, y: u16, width: u16, height: u16, title: &str) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(x, y), Print('┌')).unwrap();
    execute!(stdout, MoveTo(x + width - 1, y), Print('┐')).unwrap();
    execute!(stdout, MoveTo(x, y + height - 1), Print('└')).unwrap();
    execute!(stdout, MoveTo(x + width - 1, y + height - 1), Print('┘')).unwrap();

    for i in 1..width - 1 {
        execute!(stdout, MoveTo(x + i, y), Print('─')).unwrap();
        execute!(stdout, MoveTo(x + i, y + height - 1), Print('─')).unwrap();
    }

    for i in 1..height - 1 {
        execute!(stdout, MoveTo(x, y + i), Print('│')).unwrap();
        execute!(stdout, MoveTo(x + width - 1, y + i), Print('│')).unwrap();
    }

    if !title.is_empty() {
        let title = format!(" {} ", title);
        execute!(stdout, MoveTo(x + 2, y), Print(title)).unwrap();
    }
}

pub fn music_player(playlist_name: Option<String>) {
    // Configuração do handler para Ctrl+C
    ctrlc::set_handler(move || {
        disable_raw_mode().unwrap();
        std::process::exit(0);
    }).unwrap();

    let mut playlists: HashMap<String, Vec<Song>> = HashMap::new();
    let music_dir = Path::new("music");
    
    if !music_dir.exists() {
        fs::create_dir_all(music_dir).unwrap();
    }

    let mut songs = Vec::new();
    if let Ok(entries) = fs::read_dir(music_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().map(|ext| ext == "mp3").unwrap_or(false) {
                let name = path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                if let Ok(file) = File::open(&path) {
                    if let Ok(decoder) = Decoder::new(file) {
                        let duration = decoder.total_duration().unwrap_or_else(|| {
                            Duration::from_secs(180) // 3 minutos padrão se não conseguir ler
                        });
                        songs.push(Song::new(
                            name,
                            path.to_str().unwrap().to_string(),
                            duration
                        ));
                    }
                }
            }
        }
    }

    if songs.is_empty() {
        println!("No songs found in 'music' directory.");
        return;
    }

    playlists.insert("default".to_string(), songs.clone());
    let current_playlist = playlist_name.unwrap_or_else(|| "default".to_string());
    let song_list = playlists.get(&current_playlist).cloned().unwrap_or(songs);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut current_song = 0;
    let mut selected = 0;
    let mut viewport_start = 0;
    let mut volume: f32 = 0.5;
    let mut total_elapsed = Duration::default();
    let mut play_start: Option<Instant> = None;
    let mut playing = true;

    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let mut last_frame = Instant::now();
    loop {
        // Controle de FPS (20 FPS)
        if last_frame.elapsed() < Duration::from_millis(50) {
            thread::sleep(Duration::from_millis(10));
            continue;
        }
        last_frame = Instant::now();

        let (cols, rows) = size().unwrap();
        let box_width = 50.min(cols);
        let box_height = 12.min(rows - 6);

        execute!(stdout, Clear(ClearType::All)).unwrap();

        // Header
        execute!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(Color::Cyan),
            Print(format!(
                "Climusic Player v3.0 beta | Status: {} | Volume: {}%",
                if playing { "▶ Playing" } else { "⏸ Paused" },
                (volume * 100.0) as u8
            )),
            ResetColor
        ).unwrap();

        // Current song info
        if !song_list.is_empty() {
            let current = &song_list[current_song];
            let elapsed = if playing {
                total_elapsed + play_start.map(|s| s.elapsed()).unwrap_or_default()
            } else {
                total_elapsed
            };

            let total_duration = current.duration.as_secs() as f32;
            let progress = if total_duration > 0.0 {
                elapsed.as_secs() as f32 / total_duration
            } else {
                0.0
            };

            execute!(
                stdout,
                MoveTo(0, 2),
                SetForegroundColor(Color::Magenta),
                Print(format!("Now Playing: {}", current.name)),
                ResetColor
            ).unwrap();

            execute!(
                stdout,
                MoveTo(0, 3),
                Print(format!(
                    "[{} / {}]",
                    format_duration(elapsed),
                    format_duration(current.duration)
                ))
            ).unwrap();

            // Progress bar
            let width = 30;
            let filled = (progress * width as f32) as usize;
            let bar = format!("[{}{}]", "▰".repeat(filled), "▱".repeat(width - filled));
            execute!(
                stdout,
                MoveTo(0, 4),
                SetForegroundColor(Color::Green),
                Print(bar),
                ResetColor
            ).unwrap();
        }

        // Song list box
        draw_box(0, 5, box_width, box_height, "Queue list");
        let display_count = (box_height - 2) as usize;
        let end = (viewport_start + display_count).min(song_list.len());
        let display_songs = &song_list[viewport_start..end];

        for (i, song) in display_songs.iter().enumerate() {
            let y_pos = 6 + i as u16;
            let song_index = viewport_start + i;
            let is_current = song_index == current_song;
            let is_selected = song_index == selected;

            let mut display = String::new();
            if is_current {
                display.push_str("▶ ");
            }
            display.push_str(&song.name);

            execute!(stdout, MoveTo(1, y_pos)).unwrap();
            if is_selected {
                execute!(stdout, SetBackgroundColor(Color::Blue), SetForegroundColor(Color::White)).unwrap();
            } else if is_current {
                execute!(stdout, SetForegroundColor(Color::Green)).unwrap();
            }
            
            execute!(stdout, Print(&display), ResetColor).unwrap();
        }

        // Help text
        let help_text = "[↑/↓] Navigate | [Space] Play/Pause | [+/-] Volume | [N] Next | [Q] Quit";
        execute!(stdout, MoveTo(0, rows - 1), Print(help_text)).unwrap();

        stdout.flush().unwrap();

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                            if selected < viewport_start {
                                viewport_start = selected;
                            }
                        }
                    },
                    KeyCode::Down => {
                        if selected < song_list.len() - 1 {
                            selected += 1;
                            if selected >= viewport_start + display_count {
                                viewport_start = selected - display_count + 1;
                            }
                        }
                    },
                    KeyCode::Enter => {
                        sink.stop();
                        if let Ok(file) = File::open(&song_list[selected].path) {
                            if let Ok(decoder) = Decoder::new(file) {
                                sink.append(decoder);
                                current_song = selected;
                                playing = true;
                                play_start = Some(Instant::now());
                                total_elapsed = Duration::default();
                            }
                        }
                    },
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
                    },
                    KeyCode::Char('+') if volume < 1.0 => {
                        volume = (volume + 0.1).min(1.0);
                        sink.set_volume(volume);
                    },
                    KeyCode::Char('-') if volume > 0.0 => {
                        volume = (volume - 0.1).max(0.0);
                        sink.set_volume(volume);
                    },
                    KeyCode::Char('n') => {
                        current_song = (current_song + 1) % song_list.len();
                        sink.stop();
                        if let Ok(file) = File::open(&song_list[current_song].path) {
                            if let Ok(decoder) = Decoder::new(file) {
                                sink.append(decoder);
                                play_start = Some(Instant::now());
                                total_elapsed = Duration::default();
                                selected = current_song;
                            }
                        }
                    },
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if sink.empty() && playing && !song_list.is_empty() {
            current_song = (current_song + 1) % song_list.len();
            if let Ok(file) = File::open(&song_list[current_song].path) {
                if let Ok(decoder) = Decoder::new(file) {
                    sink.append(decoder);
                    play_start = Some(Instant::now());
                    total_elapsed = Duration::default();
                    selected = current_song;
                }
            }
        }
    }

    sink.stop();
    disable_raw_mode().unwrap();
}
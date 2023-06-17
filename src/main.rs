use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io};
use crossbeam::channel::{self, Sender, Receiver};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use invaders::frame::{Frame, Drawable};
use invaders::player::Player;
use invaders::{frame, render};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");
    
    // Start-up
    audio.play("startup");
    
    // Terminal
    let mut output = io::stdout();
    terminal::enable_raw_mode().unwrap(); // Crashes if there is an error
    output.execute(EnterAlternateScreen).unwrap();
    output.execute(Hide).unwrap();
    
    // Render loop
    let (render_tx, render_rx): (Sender<Frame>, Receiver<Frame>) = channel::unbounded();
    let renderer_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut output = io::stdout();
        render::render(&mut output, &last_frame, &last_frame, true);
        for cur_frame in render_rx {
            render::render(&mut output, &last_frame, &cur_frame, false);
            last_frame = cur_frame.clone();
        }
    });
    
    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut cur_frame = frame::new_frame();
        
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    _ => {}
                }
            }
        }
        
        // Updates
        player.update(delta);
        
        // Draw & render
        player.draw(&mut cur_frame);
        let _ = render_tx.send(cur_frame); // Ignore the error
        thread::sleep(Duration::from_millis(1));
    }
    
    
    // Exiting
    drop(render_tx);
    renderer_handle.join().unwrap();
    audio.wait();
    output.execute(Show).unwrap();
    output.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    Ok(())
}

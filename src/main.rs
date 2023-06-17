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
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }
        
        // Draw & render
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

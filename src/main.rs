use {
    std::sync:: { mpsc, Mutex, Arc },
    rdev::      { Key:: { self, * }, listen, Event, EventType },
    std::       { time::Duration, thread },
    sdl2::      {
        rect::  { Point, Rect }, surface:: { self, Surface },
        pixels:: Color, image::{ self, LoadSurface }
    }
};

const SHOW_CHARACTERS: usize = 10;

fn main() {
    let (tx, rx)      = mpsc::channel::<String>();
    let result_string = Arc::new(Mutex::new(String::new()));

    let rdev_thread   = thread::spawn(move || {
        if let Err(_) = listen(move |ev: Event| {
            if let EventType::KeyPress(code) = ev.event_type {

                let result_string = Arc::clone(&result_string);
                let mut result    = get_string(code);
                let mut unlocked_result_string = result_string.lock().expect("lock not acquired by `result_string`");

                if result == "backspace"              { unlocked_result_string.pop(); result = String::new(); }
                unlocked_result_string.push_str(&result);
                if unlocked_result_string.is_empty()  { unlocked_result_string.push_str(" "); } // --underflow
                if unlocked_result_string.len() > 500 { unlocked_result_string.drain(..500);  } // --overflow
                let result_str: String = unlocked_result_string
                    .chars().rev()
                    .take(SHOW_CHARACTERS)
                    .collect::<Vec<char>>()
                    .into_iter()
                    .rev()
                    .collect();
                
                tx.send(result_str).expect("Error communicating with mpsc");
            }
        }) { eprintln!("Error accessing keyboard"); } 
    });

    let sdl_context     = sdl2::init().unwrap();
    let ttf_context     = sdl2::ttf::init().expect("--features ttf disabled");
    let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem for SDL context");
    let mut window      = video_subsystem
        .window(" ", 300, 100)
        .position_centered()
        // .borderless() -- somehow removing borders crashes the program(why?)
        .build()
        .expect("Could not initialize window");

    window.set_always_on_top(true);

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let font = "Iosevka.ttf"; // -- put 'Iosevka.ttf' in ../src/
    let font = ttf_context.load_font(font, 42)
        .expect(&format!("Could not load font at {}", font));

    let mut previous = String::from(" ");
    'sdl: loop {
            match rx.try_recv() {
            
                Ok(result) => {

                    previous = result.clone();
                    if result.contains(".;.;.;") { break 'sdl; }
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();

                    let surface = font
                        .render(&result)
                        .blended(Color::RGB(255, 255, 255))
                        .unwrap();

                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();

                    let target = Rect::new(20, 15, surface.width(), surface.height());
                    canvas.copy(&texture, None, Some(target)).unwrap();
                    canvas.present();
                }

                Err(_) => {
                    
                    let result = previous.clone();
                    if result.contains(".;.;.;") { break 'sdl; }
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();

                    let surface = font
                        .render(&result)
                        .blended(Color::RGB(255, 255, 255))
                        .unwrap();

                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();

                    let target = Rect::new(20, 13, surface.width(), surface.height());
                    canvas.copy(&texture, None, Some(target)).unwrap();
                    canvas.present();
                
                }
        
            }
    }

     // rdev_thread.join().unwrap();
}


fn get_string(code: Key) -> String {
    match code {
        Alt | AltGr=> "Alt".to_string(),
        Backspace => "backspace".to_string(),
        CapsLock => "Cap".to_string(),
        ControlLeft => "C-L".to_string(),
        ControlRight => "C-R".to_string(),
        Delete => "Del".to_string(),
        DownArrow => "↓".to_string(),
        End => "End".to_string(),
        Escape => "Esc".to_string(),
        F1 => "F1".to_string(),
        F2 => "F2".to_string(),
        F3 => "F3".to_string(),
        F4 => "F4".to_string(),
        F5 => "F5".to_string(),
        F6 => "F6".to_string(),
        F7 => "F7".to_string(),
        F8 => "F8".to_string(),
        F9 => "F9".to_string(),
        F10 => "F10".to_string(),
        F11 => "F11".to_string(),
        F12 => "F12".to_string(),
        Home => "Home".to_string(),
        LeftArrow => "←".to_string(),
        MetaLeft => "Win".to_string(),
        MetaRight => "Win".to_string(),
        PageDown => "⇟".to_string(),
        PageUp => "⇞".to_string(),
        Return => "<-|".to_string(),
        RightArrow => "→".to_string(),
        ShiftLeft | ShiftRight => "$".to_string(),
        Space => " ".to_string(),
        Tab => "|->".to_string(),
        UpArrow => "↑".to_string(),
        PrintScreen => "Prt-Scr".to_string(),
        NumLock => "N-L".to_string(),
        BackQuote => "`".to_string(),
        Num1 => "1".to_string(),
        Num2 => "2".to_string(),
        Num3 => "3".to_string(),
        Num4 => "4".to_string(),
        Num5 => "5".to_string(),
        Num6 => "6".to_string(),
        Num7 => "7".to_string(),
        Num8 => "8".to_string(),
        Num9 => "9".to_string(),
        Num0 => "0".to_string(),
        Minus => "-".to_string(),
        Equal => "=".to_string(),
        KeyQ => "Q".to_string(),
        KeyW => "W".to_string(),
        KeyE => "E".to_string(),
        KeyR => "R".to_string(),
        KeyT => "T".to_string(),
        KeyY => "Y".to_string(),
        KeyU => "U".to_string(),
        KeyI => "I".to_string(),
        KeyO => "O".to_string(),
        KeyP => "P".to_string(),
        LeftBracket => "[".to_string(),
        RightBracket => "]".to_string(),
        KeyA => "A".to_string(),
        KeyS => "S".to_string(),
        KeyD => "D".to_string(),
        KeyF => "F".to_string(),
        KeyG => "G".to_string(),
        KeyH => "H".to_string(),
        KeyJ => "J".to_string(),
        KeyK => "K".to_string(),
        KeyL => "L".to_string(),
        SemiColon => ";".to_string(),
        Quote => "'".to_string(),
        BackSlash => "\\".to_string(),
        IntlBackslash => "\\".to_string(),
        KeyZ => "Z".to_string(),
        KeyX => "X".to_string(),
        KeyC => "C".to_string(),
        KeyV => "V".to_string(),
        KeyB => "B".to_string(),
        KeyN => "N".to_string(),
        KeyM => "M".to_string(),
        Comma => ",".to_string(),
        Dot => ".".to_string(),
        Slash => "/".to_string(),
        Insert => "Insert".to_string(),
        KpReturn => "Enter".to_string(),
        KpMinus => "-".to_string(),
        KpPlus => "+".to_string(),
        KpMultiply => "*".to_string(),
        KpDivide => "/".to_string(),
        Kp0 => "0".to_string(),
        Kp1 => "1".to_string(),
        Kp2 => "2".to_string(),
        Kp3 => "3".to_string(),
        Kp4 => "4".to_string(),
        Kp5 => "5".to_string(),
        Kp6 => "6".to_string(),
        Kp7 => "7".to_string(),
        Kp8 => "8".to_string(),
        Kp9 => "9".to_string(),
        KpDelete => "Del".to_string(),
        Function => "Fn".to_string(),
        _ => format!("{:?}", code),
    }
}

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    input::{input, InputEvent, KeyEvent},
    queue,
    screen::RawScreen,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, Output, Result,
};
use std::time::{Duration, Instant};
use rand::Rng;
use std::io::{stdout, Write};
use std::vec::IntoIter;
use std::char;
use std::cmp::{min, max};
use terminal_size::{terminal_size, Height, Width};

fn main() -> Result<()> {

    // Setup for cross term 
    clear();
    let mut gravity: u64 = 1170;
    let _raw = RawScreen::into_raw_mode()?;
    let input = input();
    let mut sync_stdin = input.read_async();
    execute!(stdout(), Hide)?;

    let (width, height) = get_terminal_size();
    let mut window = Window::new(width, height, width - 2);

    // Loop for rain particles
    loop {
        window.render();
        stdout().flush()?;
        if let Some(key_event) = sync_stdin.next() {
            let key = process_input_event(key_event);
            match key {
                Keys::ESC | Keys::Q => {
                    execute!(stdout(), Show)?;
                    // clear();
                    break;
                }
                Keys::DOWN => {
                    if gravity != 9000 {
                        gravity += 10;
                    }
                }
                Keys::UP => {
                    if gravity != 0 {
                        gravity -= 10;
                    }
                }
                Keys::NONE => {}
            }
        }
       //sleep(gravity);
    }
    Ok(())
}

fn clear() {
    execute!(
        stdout(),
        MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )
    .unwrap();
}

fn sleep(duration: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration));
}

fn ran_num(min: u16, max: u16) -> u16 {
    rand::thread_rng().gen_range(min, max)
}

fn ran_ch() -> char {
    // This gets a Jap Char.
    let c: u32 = rand::thread_rng().gen_range(65382, 65437);
    //let c: u32 = rand::thread_rng().gen_range(48, 50);
    char::from_u32(c).unwrap()
}

fn ran_color() -> RainColor {
        // Colors for Rain
        let green = RainColor(Color::White, Color::Green);
        let dark_green = RainColor(Color::DarkGrey, Color::DarkGreen);
        let grey = RainColor(Color::DarkGrey, Color::Grey);
        let colors = vec![green, dark_green, grey];
        let num = ran_num(0, 3);
        colors.get(num as usize).unwrap().clone()
}

fn clamp(n: u8, minn: u8, maxn: u8) -> u8 {
       max(min(maxn, n), minn) 
}

fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        (w, h)
    } else {
        (20, 20)
    }
}

fn process_input_event(key_event: InputEvent) -> Keys {
    match key_event {
        InputEvent::Keyboard(k) => match k {
            KeyEvent::Char(c) => match c {
                'q' => return Keys::Q,
                _ => {}
            },
            KeyEvent::Esc => return Keys::ESC,
            KeyEvent::Up => return Keys::UP,
            KeyEvent::Down => return Keys::DOWN,
            _ => {}
        },
        _ => {}
    }
    Keys::NONE
}

fn render(ch: Option<char>, fg: Color, bg: Color, x: u16, y: u16) {
    match ch {
        Some(c) => {
            queue!(
                stdout(),
                MoveTo(x, y),
                SetForegroundColor(fg),
                SetBackgroundColor(bg),
                Output(c),
                ResetColor
            )
            .unwrap();
        }
        None => {},
    }
}

enum Keys {
    ESC,
    UP,
    DOWN,
    Q,
    NONE,
}

#[derive(Copy, Clone)]
struct RainColor(Color, Color);

struct Rain {
    x: u16,
    front_y: u16,
    chars: Vec<char>,
    sec_y: u16,
    length: u16,
    height: u16,
    color: RainColor,
    delay: u64,
    speed: Duration,
}

impl Rain {
    fn new(loc: u16, height: u16, length: u16, color: RainColor, time: Instant) -> Self {
        let mut chars = Vec::new();
        let d = ran_num(50, 400) as u64;
        chars.extend(std::iter::repeat_with(ran_ch).take(height as usize));
        Rain {
            x: loc,
            front_y: 0,
            chars: chars,
            sec_y: 0,
            length: length,
            height: height,
            color: color,
            delay: d,
            speed: Duration::from_millis(d) + time.elapsed(),
        }
    }

    fn render(&mut self) {
        let mut rgbv: Vec<u8> = Vec::new();
        for x in 0..self.length + 1 {
            rgbv.insert(0, clamp((255 - (x * (255 / self.length)) as u8), 0, 255));
        }
        let mut clone_rgbv = rgbv.clone();
        let mut g = clone_rgbv.into_iter();

        let mut color_idxs = Rain::trail(Rain::loc_finder(self.length, self.sec_y), self.length).into_iter();
        for (idx, pix) in self.chars.iter().enumerate() {

            if idx <= self.sec_y as usize && self.sec_y < self.length {

                render(Some(*pix), Color::Rgb{r:0, g:rgbv[color_idxs.next().unwrap_or(0)], b:0}, Color::Black, self.x, idx as u16);

            } else {

                if self.front_y != 0  && idx < (self.front_y as usize) {
                    if (idx as u16) < self.front_y && (idx as i16) > ((self.front_y as i16) - (self.length as i16)) {
                        render(Some(*pix), Color::Rgb{r:0, g:g.next().unwrap_or(0), b:0}, Color::Black, self.x, idx as u16)
                    }
                }
                else if idx == (self.front_y as usize) {
                    render(Some(*pix), Color::White, Color::Black, self.x, self.front_y);
                }
            }
        }
        if self.front_y >= self.length {
            render(Some(' '), Color::Black, Color::Black, self.x, self.front_y - self.length);
        }
        self.sec_y = self.front_y.clone(); 
        self.front_y += 1;
    }

    fn is_empty(&self) -> bool {
        if self.front_y > self.length {
            if self.front_y - self.length == self.height {
                return true
            }
            return false
        }
        false
    }

    fn loc(&self) -> usize {
        self.x as usize
    }

    fn update(&mut self, time: Instant) {
        self.speed = Duration::from_millis(self.delay) + time.elapsed();
    }

    fn loc_finder(length: u16, y: u16) -> u16 {
        let l = length as i16;
        let y = y as i16;
        let x = l - y - 1;
        if x >= 0 {
            x as u16
        } else {
            0
        }
    }

    fn trail(loc: u16, length: u16) -> Vec<usize> {
        let mut result = Vec::new();
        (0..length).for_each(|i| if i >= loc { result.push(i as usize) } );
        //result.reverse();
        result
    }

}

struct Window {
    width: u16,
    height: u16,
    amount: u16,
    displayed: Vec<Rain>,
    now: Instant,
}

impl Window {
    fn new(width: u16, height: u16, amount: u16) -> Self {
        let time = Instant::now();
        let mut rain = Vec::with_capacity(height as usize);
        (0..amount).for_each(|_| rain.push(Rain::new(ran_num(0, width), height, ran_num(10, height-15), ran_color(), time)));

        Window {
            width,
            height,
            amount,
            displayed: rain,
            now: time,
        }
    }

    fn render(&mut self) {
        let mut keep: Vec<bool> = Vec::new();
        let mut i = 0;

        for rain in self.displayed.iter_mut() {
            if rain.speed < self.now.elapsed() {
                rain.render();
                rain.update(self.now);
            }
            if rain.is_empty() {
                keep.push(false);
            } else {
                keep.push(true);
            }
        }

        self.displayed.retain(|_| (keep[i], i += 1).0);
        if self.displayed.len() < (self.amount as usize) {
            let amount = self.amount - (self.displayed.len() as u16);
            let mut loc: u16;
            if amount != 0 {
                for _ in 0..amount {
                    
                    self.displayed
                         .push(
                             Rain::new(
                                 self.get_row(),
                                 self.height,
                                 ran_num(10, self.height-5),
                                 ran_color(),
                                 self.now,
                                 )
                             );
                }
            }
        }
    }

    fn get_row(&self) -> u16 {
        let mut locs: Vec<usize> = Vec::with_capacity(self.displayed.len());
        for rain in self.displayed.iter() {
           locs.push(rain.loc()); 
        } 
        let num = loop {
        let n = ran_num(0, self.width) as usize;
        if !locs.contains(&n) { break n; }
        };
        num as u16
    }
}



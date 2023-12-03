use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color (pub u8, pub u8, pub u8);

#[derive(Debug, Clone)]
pub struct Terminal {
    width: usize,
    height: usize,

    ch: Box<[char]>,
    fg: Box<[Color]>,
    bg: Box<[Color]>
}

impl Terminal {
    pub fn new() -> Self {
        let mut win_size: libc::winsize = unsafe {std::mem::zeroed()};
        unsafe {libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut win_size)};

        let columns = win_size.ws_col as usize;
        let rows = win_size.ws_row as usize;


        let mut ch_buf = Vec::new();
        ch_buf.resize(columns*rows, ' ');


        let mut fg_buf = Vec::new();
        fg_buf.resize(columns*rows, Color(0, 0, 0));

        let mut bg_buf = Vec::new();
        bg_buf.resize(columns*rows, Color(0, 0, 0));


        Self {
            width: columns,
            height: rows,
            ch: ch_buf.into_boxed_slice(),
            fg: fg_buf.into_boxed_slice(),
            bg: bg_buf.into_boxed_slice()
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, ch: char) {
        if !self.is_valid_coords(x, y) {return};
        self.ch[y*self.width+x] = ch;
    }

    pub fn set_color_char(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        if !self.is_valid_coords(x, y) {return};
        self.ch[y*self.width+x] = ch;
        self.fg[y*self.width+x] = fg;
        self.bg[y*self.width+x] = bg;
    }

    pub fn set_fg(&mut self, x: usize, y: usize, color: Color) {
        if !self.is_valid_coords(x, y) {return};
        self.fg[y*self.width+x] = color;
    }

    pub fn set_bg(&mut self, x: usize, y: usize, color: Color) {
        if !self.is_valid_coords(x, y) {return};
        self.bg[y*self.width+x] = color;
    }

    pub fn clear(&mut self, fg: Color, bg: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.ch[y*self.width+x] = ' ';
                self.fg[y*self.width+x] = fg;
                self.bg[y*self.width+x] = bg;
            }
        }
    }

    pub fn display(&self) {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let fg = self.fg[y*self.width+x];
                let bg = self.bg[y*self.width+x];

                output += format!("\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}", fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, self.ch[y*self.width+x]).as_str();
            }
            if y < self.height-1 {output += "\n"};
        }

        print!("{}\x1b[m", output);

        let _ = std::io::stdout().flush();
    }

    fn is_valid_coords(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
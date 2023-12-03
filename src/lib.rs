use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color (pub u8, pub u8, pub u8);

pub struct Terminal {
    width: usize,
    height: usize,

    ch: Box<[Box<[char]>]>,
    fg: Box<[Box<[Color]>]>,
    bg: Box<[Box<[Color]>]>
}

impl Terminal {
    pub fn new() -> Self {
        let mut win_size: libc::winsize = unsafe {std::mem::zeroed()};
        unsafe {libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut win_size)};

        let columns = win_size.ws_col as usize;
        let rows = win_size.ws_row as usize;


        let mut ch_row = Vec::new();
        ch_row.resize(columns, ' ');

        let mut ch_buf = Vec::new();
        ch_buf.resize(rows, ch_row.clone().into_boxed_slice());


        let mut color_row = Vec::new();
        color_row.resize(columns, Color(0, 0, 0));

        let mut fg_buf = Vec::new();
        fg_buf.resize(rows, color_row.clone().into_boxed_slice());

        let mut bg_buf = Vec::new();
        bg_buf.resize(rows, color_row.clone().into_boxed_slice());


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
        self.ch[y][x] = ch;
    }

    pub fn set_color_char(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        if !self.is_valid_coords(x, y) {return};
        self.ch[y][x] = ch;
        self.fg[y][x] = fg;
        self.bg[y][x] = bg;
    }

    pub fn set_fg(&mut self, x: usize, y: usize, color: Color) {
        if !self.is_valid_coords(x, y) {return};
        self.fg[y][x] = color;
    }

    pub fn set_bg(&mut self, x: usize, y: usize, color: Color) {
        if !self.is_valid_coords(x, y) {return};
        self.bg[y][x] = color;
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.ch[y][x] = ' ';
                self.fg[y][x] = Color(0, 0, 0);
                self.bg[y][x] = Color(0, 0, 0);
            }
        }
    }

    pub fn display(&self) {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let fg = self.fg[y][x];
                let bg = self.bg[y][x];

                output += format!("\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}", fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, self.ch[y][x]).as_str();
            }
            output += "\n"
        }

        print!("{}\x1b[m", output.trim_end());

        let _ = std::io::stdout().flush();
    }

    fn is_valid_coords(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
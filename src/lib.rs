use std::io::Write;

pub struct Terminal {
    width: usize,
    height: usize,

    buf: Box<[Box<[char]>]>
}

impl Terminal {
    pub fn new() -> Self {
        let mut win_size: libc::winsize = unsafe {std::mem::zeroed()};
        unsafe {libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut win_size)};

        let columns = win_size.ws_col as usize;
        let rows = win_size.ws_row as usize;

        let mut row = Vec::new();
        row.resize(columns, ' ');

        let mut arr = Vec::new();
        arr.resize(rows, row.clone().into_boxed_slice());

        Self {
            width: columns,
            height: rows,
            buf: arr.into_boxed_slice()
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, ch: char) {
        if !self.is_valid_coords(x, y) {return};
        self.buf[x][y] = ch;
    }

    pub fn clear(&mut self) {
        self.buf.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|ch| {
                *ch = ' ';
            })
        });
    }

    pub fn display(&self) {
        self.buf.iter().enumerate().for_each(|(index, chars)| {
            

            if index < self.height-1 {
                print!("{}\n", chars.iter().collect::<String>());
            }
            else {
                print!("{}", chars.iter().collect::<String>());
            }
        });

        let _ = std::io::stdout().flush();
    }

    fn is_valid_coords(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
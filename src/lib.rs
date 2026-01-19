use point::Point;
use std::collections::HashMap;
use std::fmt;

/// A window to be displayed. Do note that the origin is the top left
/// corner of the terminal.
#[derive(Clone, Debug)]
pub struct Window<T: fmt::Display> {
    /// Top left co-ordinate of the window.
    pub top_left: Point,
    /// Contains all the characters of the window in rows.
    pub data: Vec<Vec<T>>,
}

#[allow(unused_must_use)]
impl<T: fmt::Display> Window<T> {
    /// Create a new empty window at the given position.
    pub fn new(top_left: Point) -> Self {
        Self {
            top_left,
            data: Vec::new(),
        }
    }

    /// Draws an outline around the window
    /// with the given character.
    /// If the data is not rectangular, the outline won't be either.
    pub fn outline_with(&mut self, ch: T)
    where
        T: Clone,
    {
        // Add characters to start and end of each row.
        for row in self.data.iter_mut() {
            row.insert(0, ch.clone());
            row.push(ch.clone());
        }

        // Create bonus rows above and below the rest of the window.
        let first_len = self.data[0].len();
        let last_len = self.data.last().unwrap().len();

        self.data.insert(0, vec![ch.clone(); first_len]);
        self.data.push(vec![ch; last_len]);
    }
}

impl<T: fmt::Display> fmt::Display for Window<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            for ch in row.iter() {
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Write for Window<char> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let mut len = self.data.len();
        if len == 0 {
            self.data.push(Vec::new());
            len += 1;
        }
        let mut cur_idx = len - 1;

        for line in s.lines() {
            self.data[cur_idx].extend(line.chars());
            cur_idx += 1;
            self.data.push(Vec::new());
        }

        Ok(())
    }
}

/// Contains various windows and displays them according to their position.
#[derive(Clone, Debug, Default)]
pub struct Container<T: fmt::Display> {
    /// Each window stored in the container.
    pub windows: Vec<Window<T>>,
    buffer: HashMap<Point, T>,
    // Contains all positions that have been changed since the last refresh.
    changed: Vec<Point>,
}

impl<T: fmt::Display> Container<T> {
    /// Create an empty container.
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            buffer: HashMap::new(),
            changed: Vec::new(),
        }
    }

    /// Add the given window to the container.
    pub fn add_win(&mut self, win: Window<T>) {
        self.windows.push(win);
    }

    /// Return a slice of all positions in the buffer that have been changed since the last call to refresh.
    pub fn changed(&self) -> &[Point] {
        &self.changed
    }

    /// Return a reference to the internal buffer.
    pub fn get_buffer(&self) -> &HashMap<Point, T> {
        &self.buffer
    }

    /// Redraws all the windows into the buffer.
    pub fn refresh(&mut self)
    where
        T: Clone + PartialEq,
    {
        self.buffer.clear();
        self.changed = Vec::new();

        for win in self.windows.iter() {
            for (y, row) in win.data.iter().enumerate() {
                for (x, ch) in row.iter().enumerate() {
                    let p = Point::new(x as i32, y as i32) + win.top_left;

                    let prev = self.buffer.get(&p);
                    if prev.is_none() || prev.unwrap() != ch {
                        self.buffer.insert(p, ch.clone());
                        self.changed.push(p);
                    }
                }
            }
        }
    }

    /// Draws the buffer to the screen. Uses the default value of T when there is no stored
    /// value in the buffer.
    pub fn draw(&self, wid: u16, hgt: u16)
    where
        T: Default + Clone,
    {
        self.draw_with_default(wid, hgt, T::default())
    }

    /// Draws the buffer to the screen, using the provided default where there is no stored
    /// value in the buffer.
    pub fn draw_with_default(&self, wid: u16, hgt: u16, default: T)
    where
        T: Clone,
    {
        let wid = wid as i32;
        let hgt = hgt as i32;

        for y in 0..hgt {
            for x in 0..wid {
                let p = Point::new(x, y);

                print!(
                    "{}",
                    if let Some(c) = self.buffer.get(&p) {
                        c.clone()
                    } else {
                        default.clone()
                    }
                );
            }
            println!();
        }
    }

    /// Creates a string representation of the container with positions from (0, 0) to
    /// (wid, hgt), using the provided default when there is no stored value in the
    /// buffer.
    pub fn to_string_with_default(&self, wid: u16, hgt: u16, default: T) -> String
    where
        T: Clone,
    {
        let mut out = String::new();
        let wid = wid as i32;
        let hgt = hgt as i32;

        for y in 0..hgt {
            for x in 0..wid {
                let p = Point::new(x, y);

                let ch = if let Some(c) = self.buffer.get(&p) {
                    c.clone()
                } else {
                    default.clone()
                };

                out.push_str(&ch.to_string());
            }
            out.push('\n');
        }

        out
    }

    /// Creates a string representation of the container with positions from (0, 0) to
    /// (wid, hgt), using the default value of T when there is no stored value in the
    /// buffer.
    pub fn to_string(&self, wid: u16, hgt: u16) -> String
    where
        T: Clone + Default,
    {
        self.to_string_with_default(wid, hgt, T::default())
    }
}

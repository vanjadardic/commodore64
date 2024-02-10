pub enum Key {
    InsertDelete,
    N3,
    N5,
    N7,
    N9,
    Plus,
    Pound,
    N1,
    Return,
    W,
    R,
    Y,
    I,
    P,
    Asterisk,
    LeftArrow,
    CursorLeftRight,
    A,
    D,
    G,
    J,
    L,
    Semicolon,
    Control,
    F7,
    N4,
    N6,
    N8,
    N0,
    Minus,
    ClearHome,
    N2,
    F1,
    Z,
    C,
    B,
    M,
    Period,
    RightShift,
    Space,
    F3,
    S,
    F,
    H,
    K,
    Colon,
    Equal,
    Commodore,
    F5,
    E,
    T,
    U,
    O,
    At,
    UpArrow,
    Q,
    CursorUpDown,
    LeftShift,
    X,
    V,
    N,
    Comma,
    Slash,
    RunStop,
    Restore,
}


pub struct Keyboard {
    pressed: [bool; 64],
    // col: u8,
    // row: u8,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed: [false; 64],
            // col: 0,
            // row: 0,
        }
    }

    pub fn change_key_state(&mut self, key: Key, pressed: bool) {
        // let a = ;
        self.pressed[key as usize] = pressed;
        // self.col = 0;
        // self.row = 0;
        // for (i, pressed) in self.pressed.iter().enumerate() {
        //     if *pressed {
        //         self.col |= 1 << (i % 8);
        //         self.row |= 1 << (i / 8);
        //     }
        // }
        // debug!("key={} pressed={} col={}, row={}", a, pressed, self.col, self.row);
    }


    pub fn pressed(&self) -> [bool; 64] {
        self.pressed
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::keyboard::Key;

    #[test]
    fn relative_addressing() {
        assert_eq!(56, Key::CursorUpDown as u8);
        assert_eq!(57, Key::LeftShift as u8);
        assert_eq!(17, Key::A as u8);
        assert_eq!(0, Key::InsertDelete as u8);
    }
}
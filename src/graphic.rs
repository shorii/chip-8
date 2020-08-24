pub struct Graphic {
    pub gfx: [u8; 2048],
}

// XXX depricated
impl Graphic {
    pub fn new() -> Self {
        Graphic { gfx: [0; 2048] }
    }
    pub fn clear(&mut self) {
        self.gfx = [0; 2048];
    }
    pub fn set_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        assert!(x <= 64);
        assert!(y <= 32);
        let mut collision = false;
        for (i, sprite_fragment) in sprite.iter().enumerate() {
            for xi in 0..8 {
                let pixel = (sprite_fragment & (0x80 >> xi)) as u8;
                let yi = y + i;
                let index = yi.checked_mul(64).unwrap()
                              .checked_add(x).unwrap()
                              .checked_add(xi).unwrap();
                if pixel != 0 {
                    let screen_pixel = self.gfx[index];
                    if screen_pixel == 1 {
                        collision = true;
                    }
                    self.gfx[index] ^= 1;
                }
            }
        }
        collision
    }
}

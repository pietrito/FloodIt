use sdl2::pixels::Color;

pub struct ColorHSV(pub f32, pub f32, pub f32);

impl ColorHSV {
    pub fn to_sdl_color(self) -> Color {
        let c = self.1 * self.2;
        let h2 = self.0 / 60f32;
        let x = c * (1f32 - f32::abs(h2 % 2f32 - 1f32));
        let m = self.2 - c;

        let (r, g, b) = match h2 as u32 {
            0 => (c + m, x + m, m),
            1 => (x + m, c + m, m),
            2 => (m, c + m, x + m),
            3 => (m, x + m, c + m),
            4 => (x + m, m, c + m),
            5 => (c + m, m, x + m),
            _ => {
                panic!("Invalid value for h2: {}", h2);
            }
        };

        Color::RGB((r * 255f32) as u8, (g * 255f32) as u8, (b * 255f32) as u8)
    }
}

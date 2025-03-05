use std::f32::consts::{PI, TAU};

pub type Scalar = u32;

pub fn inverse_lerp(x: f32, y: f32, t: f32) -> f32 {
    if x == y {
        return 0.0;
    }
    (t - x) / (y - x)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub enum Color {
    #[default]
    Default,
    Transparent,
    Rgb([u8; 3]),
}

impl Color {
    fn from_float([r, g, b]: [f32; 3]) -> Self {
        let scale = |d: f32| (255.0_f32 * d.clamp(0.0, 1.0)).round() as u8;
        Self::Rgb([scale(r), scale(g), scale(b)])
    }
}

pub struct Time {
    t: f32,
    d: f32,
}

impl Time {
    pub const fn new(d: f32) -> Self {
        Self { t: 0.0, d }
    }

    pub fn update(&mut self, dt: f32) {
        self.t += dt
    }

    pub fn normalize(&self) -> f32 {
        self.t / self.d
    }
}

pub fn vertical_wave(
    dt: f32,
    width: Scalar,
    height: Scalar,
    mut put: impl FnMut(Scalar, Scalar, Pixel),
) {
    let speed = 3.0;
    let freq = 0.05;
    let amp = width as f32 / 4.0;

    for y in 0..height {
        for x in 0..width {
            let phase = (y as f32 * freq + dt * speed).sin() * amp;
            let intensity = ((x as f32 - width as f32 / 2.0 + phase) / amp).abs();
            let intensity = (1.0 - intensity).clamp(0.0, 1.0);
            let pixel = Pixel {
                ch: ' ',
                bg: Color::from_float([intensity * 0.3, intensity * 0.3, intensity]),
                fg: Color::Default,
            };
            put(x, y, pixel)
        }
    }
}

pub fn horizontal_wave(
    dt: f32,
    width: Scalar,
    height: Scalar,
    mut put: impl FnMut(Scalar, Scalar, Pixel),
) {
    let speed = 3.0;
    let freq = 0.07;
    let amp = height as f32 / 4.0;

    for y in 0..height {
        for x in 0..width {
            let phase = ((x as f32 / 2.1) * freq + dt * speed).sin() * amp;
            let intensity = ((y as f32 - height as f32 / 2.0 + phase) / amp).abs();
            let intensity = (1.0 - intensity).clamp(0.0, 1.0);
            let pixel = Pixel {
                ch: ' ',
                bg: Color::from_float([intensity, intensity * 0.3, intensity * 0.3]),
                fg: Color::Default,
            };
            put(x, y, pixel)
        }
    }
}

pub fn pulse(dt: f32, width: Scalar, height: Scalar, mut put: impl FnMut(Scalar, Scalar, Pixel)) {
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let speed = TAU;

    for y in 0..height {
        for x in 0..width {
            let l = inverse_lerp(0.0, width as f32 - 1.0, x as f32);
            let freq = (l + dt).sin().abs() * 0.5;
            let (dx, dy) = (((x as f32 - cx) / 2.1).abs(), (y as f32 - cy).abs());
            let distance = dx.max(dy);
            let pulse = ((distance * freq) - (speed * dt)).cos().abs();
            let pixel = Pixel {
                ch: ' ',
                bg: Color::from_float([pulse * l + 0.1, pulse * l + 0.2, pulse * l + 0.3]),
                fg: Color::Default,
            };
            put(x, y, pixel)
        }
    }
}

pub fn spiral(t: f32, width: Scalar, height: Scalar, mut put: impl FnMut(Scalar, Scalar, Pixel)) {
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let arms = 6;
    let spin = 1.0;
    let factor = 0.1;

    let s = ['▮', '─', '━', '█', '╌', '╍', '═', '■'];

    for y in 0..height {
        for x in 0..width {
            let (dx, dy) = ((x as f32 - cx) / 2.1, y as f32 - cy);
            let distance = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx) + t * spin;
            let color = ((angle * arms as f32) - distance * factor).sin().abs();

            let pixel = Pixel {
                ch: s[distance.round() as usize % s.len()],
                fg: Color::from_float([color - 0.5, color * 0.5, color]),
                bg: Color::Transparent,
            };
            put(x, y, pixel)
        }
    }
}

pub fn checkerboard(
    dt: f32,
    width: Scalar,
    height: Scalar,
    mut put: impl FnMut(Scalar, Scalar, Pixel),
) {
    let pulse = 0.5;
    let w = 5;
    let h = 3;

    let (sin, cos) = dt.sin_cos();

    for y in 0..height {
        for x in 0..width {
            let theta = if (((x / w) % 2) ^ ((y / h) % 2)) == 0 {
                sin
            } else {
                cos
            };

            let l = inverse_lerp(0.0, width as f32 - 1.0, x as f32);
            let color = (pulse * theta * 0.5 + 0.5).abs() + (1.0 * l);
            let pixel = Pixel {
                ch: ' ',
                bg: Color::from_float([color + l, color - l, color * l]),
                fg: Color::Default,
            };
            put(x, y, pixel);
        }
    }
}

type Palette = Box<[[u8; 3]]>;

fn new_palette() -> Palette {
    vec![[0, 0, 0]; 256].into_boxed_slice()
}

pub struct Plasma {
    palette: Palette,
}

impl Plasma {
    pub fn new(_width: Scalar, _height: Scalar) -> Self {
        let mut palette = new_palette();

        for i in 0..255 {
            let v = i as f32 / 255.0 * 6.0 - 3.0;
            let r = ((v * PI).cos() * 255.0).floor().max(0.0);
            let g = v;
            let b = ((v * PI).sin() * 255.0).floor().max(0.0);
            palette[i] = [r as u8, g as u8, b as u8]
        }

        Self { palette }
    }

    pub fn update(&mut self, _width: Scalar, _height: Scalar) {}

    pub fn render(
        &mut self,
        dt: f32,
        width: Scalar,
        height: Scalar,
        mut put: impl FnMut(Scalar, Scalar, Pixel),
    ) {
        let (dt_sin, dt_cos) = dt.sin_cos();

        for y in 0..height {
            let dy = (y as f32 / height as f32) - 0.5;
            for x in 0..width {
                let dx = (x as f32 / width as f32) - 0.5;
                let (cx, cy) = (dx + 0.5 * dt_sin, dy + 0.5 * dt_cos);

                let mut v = (dx * 10.0 + dt).sin();
                v += ((50.0 * (cx * cx + cy * cy) + 1.0).sqrt() + dt).sin();
                v += ((dx * dx + dy * dy).sqrt() - dt).cos();
                v = ((v / 6.0 + 0.5) * 255.0).floor();

                let color = self.palette[v as usize];
                let pixel = Pixel {
                    ch: ' ',
                    fg: Color::Default,
                    bg: Color::Rgb(color),
                };
                put(x, y, pixel)
            }
        }
    }
}

pub struct Blobs {
    palette: Palette,
    shapes: Vec<Blob>,
}

struct Blob {
    sx: f32,
    sy: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Blobs {
    pub fn new(_width: Scalar, _height: Scalar) -> Self {
        let mut palette = new_palette();
        for i in 0..255 {
            let t = i as u8;
            palette[i] = [t / 8, t / 2, t / 2]
        }

        let mut shapes = vec![];
        shapes.extend(
            std::iter::repeat_with(|| Blob {
                sx: fastrand::f32() * 0.5,
                sy: fastrand::f32() * 0.5,
                speed: fastrand::f32() * PI * 32.0 - PI * 16.0,
                x: 0.0,
                y: 0.0,
            })
            .take(5),
        );

        Self { palette, shapes }
    }

    pub fn update(&mut self, _width: Scalar, _height: Scalar) {}

    pub fn render(
        &mut self,
        dt: f32,
        width: Scalar,
        height: Scalar,
        mut put: impl FnMut(Scalar, Scalar, Pixel),
    ) {
        let (cx, cy) = (width as f32 / 2.0, height as f32 / 2.0);

        let t = dt / 300.0;
        let mut shift = 0.0;
        for blob in &mut self.shapes {
            blob.x = ((t + shift) * TAU * blob.speed).sin() * width as f32 * blob.sx + cx;
            blob.y = ((t + shift) * TAU * blob.speed).cos() * height as f32 * blob.sy + cy;
            shift += 2.0;
        }

        for y in 0..height {
            for x in 0..width {
                let mut s = 1.0;
                for blob in &self.shapes {
                    let xq = x as f32 - blob.x;
                    let yq = y as f32 - blob.y;
                    s *= (xq * xq + yq * yq).sqrt()
                }

                let theta = (width as f32 * 2.0 - (s / 1e5)).floor();
                let color = theta.clamp(0.0, 255.0) as u8;
                let pixel = Pixel {
                    ch: ' ',
                    fg: Color::Default,
                    bg: Color::Rgb(self.palette[color as usize]),
                };
                put(x, y, pixel)
            }
        }
    }
}

pub struct Fire {
    palette: Palette,
    particles: Vec<u8>,
}

impl Fire {
    pub fn new(width: Scalar, height: Scalar) -> Self {
        let mut palette = new_palette();
        for i in 0..255 {
            let t = i as u8;
            palette[i] = [t, t >> 2, t >> 4]
        }

        let particles = vec![0; ((width + 2) * (height + 2)) as usize];
        Self { palette, particles }
    }

    pub fn update(&mut self, width: Scalar, height: Scalar) {
        self.particles
            .resize(((width + 2) * (height + 2)) as usize, 0);
        self.particles.fill(0);
    }

    pub fn render(
        &mut self,
        _dt: f32,
        width: Scalar,
        height: Scalar,
        mut put: impl FnMut(Scalar, Scalar, Pixel),
    ) {
        let offset = |x, y| y * width + x;

        if self.particles.len() != ((width + 2) * (height + 2)) as usize {
            self.update(width, height);
        }

        for y in 0..height {
            for x in 0..width {
                let offset = offset(x, y) as usize;
                let width = width as usize;

                let v = [
                    offset + width - 1,
                    offset + width,
                    offset + width * 2,
                    offset + width * 2 + 1,
                ]
                .iter()
                .fold(0i16, |a, &t| a + (self.particles[t] as i16));

                self.particles[offset] = ((v >> 2) - 2).abs().clamp(0, 255) as u8
            }
        }

        for x in 0..width {
            let offset = offset(x, height) as usize;
            self.particles[offset] = if fastrand::f32() > 0.7 { 255 } else { 0 };
        }

        for y in 0..height {
            for x in 0..width {
                let offset = offset(x, y) as usize;
                let color = self.palette[self.particles[offset] as usize];
                let pixel = Pixel {
                    ch: ' ',
                    fg: Color::Default,
                    bg: Color::Rgb(color),
                };
                put(x, y, pixel)
            }
        }
    }
}

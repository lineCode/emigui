#[derive(Clone, Default)]
pub struct Texture {
    /// e.g. a hash of the data. Use this to detect changes!
    pub id: u64, // TODO
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl std::ops::Index<(usize, usize)> for Texture {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Texture {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.pixels[y * self.width + x]
    }
}

/// A texture pixels, used for fonts.
#[derive(Clone, Default)]
pub struct TextureAtlas {
    texture: Texture,

    /// Used for when adding new rects
    cursor: (usize, usize),
    row_height: usize,
}

impl TextureAtlas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            texture: Texture {
                id: 0,
                width,
                height,
                pixels: vec![0; width * height],
            },
            ..Default::default()
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut Texture {
        self.texture.id += 1;
        &mut self.texture
    }

    pub fn clear(&mut self) {
        self.cursor = (0, 0);
        self.row_height = 0;
    }

    /// Returns the coordinates of where the rect ended up.
    pub fn allocate(&mut self, (w, h): (usize, usize)) -> (usize, usize) {
        assert!(w <= self.texture.width);
        if self.cursor.0 + w > self.texture.width {
            // New row:
            self.cursor.0 = 0;
            self.cursor.1 += self.row_height;
            self.row_height = 0;
        }

        self.row_height = self.row_height.max(h);
        while self.cursor.1 + self.row_height >= self.texture.height {
            self.texture.height *= 2;
        }

        if self.texture.width * self.texture.height > self.texture.pixels.len() {
            self.texture
                .pixels
                .resize(self.texture.width * self.texture.height, 0);
        }

        let pos = self.cursor;
        self.cursor.0 += w;
        self.texture.id += 1;
        (pos.0 as usize, pos.1 as usize)
    }
}

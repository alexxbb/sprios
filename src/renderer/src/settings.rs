#[derive(Copy, Clone)]
pub struct RenderSettings {
    pub width: u32,
    pub height: u32,
    pub bucket: u32,
    pub samples: u32,
}

pub struct SettingsBuilder {
    width: u32,
    height: u32,
    bucket: u32,
    samples: u32,
}

impl SettingsBuilder {
    pub fn new()-> Self {
        SettingsBuilder { width: 720, height: 480, bucket: 16, samples: 3 }
    }

    pub fn bucket(mut self, v: u32) -> Self {
        self.bucket = v;
        self
    }

    pub fn size(mut self, width: u32, height: Option<u32>) -> Self {
        self.width = width;
        self.height = height.unwrap_or((width as f32 / (16.0 / 9.0)) as u32);
        self
    }

    pub fn samples(mut self, v: u32) -> Self {
        self.samples = v;
        self
    }
    pub fn build(self) -> RenderSettings {
        RenderSettings{ width: self.width, height: self.height, bucket: self.bucket, samples: self.samples}
    }
}

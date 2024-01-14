use crate::image_helper::image_helper::ImageDataErrors;

#[derive(Debug)]
pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
      let buffer_capacity = 3_655_744;
      let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
      FloatingImage {
        width,
        height,
        data: buffer,
        name,
      }
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        // If the previously assigned buffer is too small to hold the new data
        if data.len() > self.data.capacity() {
          return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}
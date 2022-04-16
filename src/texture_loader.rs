use std::collections::HashMap;
use std::os::raw::c_void;
use std::path::Path;

use image::DynamicImage;

pub struct TextureLoader {
    textures: HashMap<String, u32>,
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        let texture_loader = TextureLoader {
            textures: HashMap::new(),
        };

        return texture_loader;
    }

    pub fn load(&mut self, path: &Path, id: &str) -> bool {
        if !path.exists() {
            println!("Invalid texture path: {}", path.to_str().unwrap());

            return false;
        }

        let mut texture = image::open(path).expect(&format!(
            "Failed to load texture: {}",
            path.to_str().unwrap()
        ));
        let format = match texture {
            DynamicImage::ImageRgb8(_) => gl::RGB,
            DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => gl::RGBA,
        };

        /********************************************************
         Texture (image file) is flipped upside down
         because coordinate of image and 3D space are different.
        ********************************************************/
        texture = texture.flipv();

        let data = texture.as_bytes().to_vec();
        let mut texture_id = 0;

        // Unsafe block to use some function of OpenGL
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, texture.width() as i32, texture.height() as i32, 0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void,);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self.textures.insert(id.to_string(), texture_id);

        return true;
    }

    pub fn get_from_id(&mut self, id: &str) -> u32 {
        return *self.textures.get(id).expect("Failed to get loaded texture");
    }
}

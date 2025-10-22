use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub images: HashMap<String, Image>,
}

impl TextureManager {
    pub fn new(rl: &RaylibHandle) -> Self {
        let mut images = HashMap::new();

        for i in 0..6 {
            let name = format!("OFF00{}", i);
            let path = format!("assets/{}.png", name);
            let image = Image::load_image(&path).expect("No se pudo cargar la imagen");
            images.insert(name.clone(), image);
        }

        TextureManager { images }
    }

    pub fn get(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}

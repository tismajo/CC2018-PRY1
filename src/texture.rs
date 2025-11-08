use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub images: HashMap<String, Image>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        let mut images = HashMap::new();
        
        for i in 0..6 {
            let name = format!("OFF00{}", i);
            let path = format!("assets/{}.png", name);
            
            match Image::load_image(&path) {
                Ok(image) => {
                    images.insert(name.clone(), image);
                    println!("Textura cargada: {}", name);
                }
                Err(e) => {
                    eprintln!("Error cargando textura {}: {:?}", name, e);
                }
            }
        }
        
        TextureManager { images }
    }
    
    pub fn get(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}

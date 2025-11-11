use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub images: HashMap<String, Image>,
}

impl TextureManager {
    pub fn new(_rl: &mut RaylibHandle) -> Self {
        let mut images = HashMap::new();

        // Carga texturas OFF000..OFF005 desde assets/OFF000.png etc.
        for i in 0..6 {
            let name = format!("OFF00{}", i);
            let path = format!("assets/{}.png", name);

            match Image::load_image(&path) {
                Ok(image) => {
                    images.insert(name.clone(), image);
                    println!("Textura cargada: {}", name);
                }
                Err(e) => {
                    eprintln!("Error cargando textura {}: {:?} (asegúrate de poner {})", name, e, path);
                }
            }
        }

        // Intento cargar sprites de enemigo/player (opcional)
        let extra = vec![("enemy", "assets/enemy.png"), ("player_anim", "assets/player_anim.png")];
        for (key, path) in extra {
            match Image::load_image(path) {
                Ok(img) => { images.insert(key.to_string(), img); println!("Sprite cargado: {}", key); },
                Err(_) => { eprintln!("No se encontró sprite {}", path); }
            }
        }

        TextureManager { images }
    }

    pub fn get(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}

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

        // Sprites del juego: claves: "F" (enemy), "C" (chest), "T" (worker/player sprite)
        let sprite_list = vec![
            ("F", "assets/enemy.png"),
            ("C", "assets/chest.png"),
            ("T", "assets/worker.png"),
        ];

        for (key, path) in sprite_list {
            match Image::load_image(path) {
                Ok(img) => {
                    images.insert(key.to_string(), img);
                    println!("Sprite cargado: {} -> {}", key, path);
                }
                Err(e) => {
                    eprintln!("No se encontró sprite {}: {:?}", path, e);
                }
            }
        }

        // Mantener compatibilidad: cargar player_anim o enemy alternativos si existen
        let extra = vec![("player_anim", "assets/player_anim.png"), ("enemy_alt", "assets/enemy_alt.png")];
        for (key, path) in extra {
            match Image::load_image(path) {
                Ok(img) => { images.insert(key.to_string(), img); println!("Sprite cargado: {}", key); },
                Err(_) => { /* silencioso */ }
            }
        }

        TextureManager { images }
    }

    pub fn get(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}

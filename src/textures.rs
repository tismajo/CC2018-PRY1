use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    images: HashMap<char, Image>,
    textures: HashMap<char, Texture2D>,
    default_texture_char: char,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        // Crear directorio assets si no existe
        if !std::path::Path::new("assets").exists() {
            if let Err(e) = std::fs::create_dir("assets") {
                eprintln!("No se pudo crear el directorio assets: {}", e);
            }
        }

        let texture_files = vec![
            ('E', "assets/OFF000.png"),
            ('#', "assets/OFF001.png"),
            ('L', "assets/OFF002.png"),
        ];

        for (ch, path) in texture_files {
            // Verificar si el archivo existe
            if !std::path::Path::new(path).exists() {
                eprintln!("Advertencia: No se encontró el archivo de textura {}", path);
                continue;
            }
            
            match Image::load_image(path) {
                Ok(image) => {
                    match rl.load_texture(thread, path) {
                        Ok(texture) => {
                            images.insert(ch, image);
                            textures.insert(ch, texture);
                            println!("Textura cargada exitosamente: {}", path);
                        },
                        Err(e) => {
                            eprintln!("Error cargando textura {}: {}", path, e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error cargando imagen {}: {}", path, e);
                }
            }
        }

        // Crear una textura por defecto simple si no hay texturas
        let default_image = Image::gen_image_color(64, 64, Color::MAGENTA);
        let default_texture = rl.load_texture_from_image(thread, &default_image)
            .expect("No se pudo crear textura por defecto");
        
        // Usar '#' como carácter por defecto si existe, sino usar el primero disponible
        let default_char = if textures.contains_key(&'#') { 
            '#' 
        } else if !textures.is_empty() {
            *textures.keys().next().unwrap()
        } else {
            '#'
        };

        // Insertar textura por defecto si no hay ninguna
        if textures.is_empty() {
            images.insert(default_char, default_image);
            textures.insert(default_char, default_texture);
            println!("Usando textura por defecto");
        }

        TextureManager { 
            images, 
            textures, 
            default_texture_char: default_char,
        }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        // Si el carácter no tiene textura, usar el color del maze como fallback
        let texture_char = if self.images.contains_key(&ch) { 
            ch 
        } else { 
            self.default_texture_char 
        };
        
        if let Some(image) = self.images.get(&texture_char) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            self.get_pixel_color_from_image(image, x, y)
        } else {
            // Fallback al color del maze si no hay textura
            crate::maze::get_cell_color(ch)
        }
    }

    fn get_pixel_color_from_image(&self, image: &Image, x: i32, y: i32) -> Color {
        let width = image.width as usize;
        let height = image.height as usize;

        if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
            return Color::MAGENTA; // Color visible para debugging
        }

        let x = x as usize;
        let y = y as usize;

        unsafe {
            let data = image.get_image_data();
            let idx = y * width + x;
            
            if idx >= width * height {
                return Color::MAGENTA;
            }

            data[idx]
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch).or_else(|| self.textures.get(&self.default_texture_char))
    }

    pub fn has_texture(&self, ch: char) -> bool {
        self.images.contains_key(&ch)
    }
    
    // Método para verificar si el manager tiene texturas cargadas
    pub fn is_initialized(&self) -> bool {
        !self.textures.is_empty()
    }
}

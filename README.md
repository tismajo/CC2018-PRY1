# 🎮 OFF (The 3D Version)

Un remake 3D del clásico juego **OFF**, creado en **Rust** con **Raylib**, inspirado en la estética original:  
laberintos oscuros, enemigos misteriosos, música ambiental y sprites pixel art.  

---
## Video funcionamiento
https://www.canva.com/design/DAG4cPrxCOk/qH_pXWMwBFt2fXGpf6c3NQ/edit?utm_content=DAG4cPrxCOk&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton

## 🧱 Características principales

- 🧭 **Exploración 3D en primera persona**
- 👹 **Enemigos (F)** que persiguen al jugador
- 👷 **Workers (T)** con comportamiento pasivo
- 💎 **Cofres (C)** que reproducen sonido y muestran *"Joker Received"*
- ❤️ Sistema de vida con overlay rojo al recibir daño
- 🔊 **Música ambiental y efectos de sonido** con [rodio](https://crates.io/crates/rodio)
- 🧩 **Varios niveles** (`maze.txt`, `maze1.txt`, `maze2.txt`)
- 🎨 Estética inspirada en el menú y atmósfera del juego OFF original

---

## 🚀 Instalación

### 1️⃣ Clonar el proyecto
```bash
git clone https://github.com/tuusuario/off-3d-version.git
cd off-3d-version
```

### 2️⃣ Instalar dependencias
Asegúrate de tener instalado **Rust** (v1.70 o superior) y **cargo**.  
Luego instala las librerías necesarias:

```bash
cargo build
```

### 3️⃣ Ejecutar
```bash
cargo run
```

---

## 🗂️ Estructura del proyecto

```
├── src/
│   ├── main.rs              # Juego principal
│   ├── audio.rs             # Sistema de sonido (rodio)
│   ├── framebuffer.rs       # Buffer y renderizado de pantalla
│   ├── maze.rs              # Lógica de carga de laberintos
│   ├── player.rs            # Movimiento y cámara del jugador
│   ├── enemy.rs             # Comportamiento de enemigos
│   ├── renderer.rs          # Renderizado 2D y 3D
│   ├── texture.rs           # Gestión de texturas
│   └── ...
│
├── assets/
│   ├── chest.png            # Sprite de cofre
│   ├── worker.png           # Sprite de worker
│   ├── enemy.png            # Sprite del enemigo
│   ├── music_background.ogg # Música ambiental
│   ├── sfx_hit.wav          # Sonido al recibir daño
│   ├── sfx_chest.wav        # Sonido al abrir cofre
│   └── menu/
│       ├── logo_x.png       # Logo del menú principal (gran X naranja)
│       ├── controls_bar.png # Barra inferior con botones
│
├── maze.txt                 # Nivel 1
├── maze1.txt                # Nivel 2
├── maze2.txt                # Nivel 3
└── README.md
```

---

## 🎮 Controles

| Acción | Tecla |
|--------|--------|
| Moverse | **W, A, S, D** |
| Girar cámara | **Ratón** |
| Siguiente nivel | **E (si aplica)** |
| Menú / Volver | **ESC** |
| Seleccionar menú | **↑ / ↓ + ENTER** |
| Reintentar tras morir | **R** |
| Salir al menú | **M** |

---

## 🔊 Audio

El sistema de sonido utiliza la librería **rodio**.  
Los sonidos y música deben estar en la carpeta `assets/`.

| Tipo | Archivo | Descripción |
|------|----------|-------------|
| Música de fondo | `music_background.ogg` | Se reproduce en bucle |
| Daño recibido | `sfx_hit.wav` | Suena al perder vida |
| Cofre | `sfx_chest.wav` | Suena una sola vez por cofre abierto |

---

## 🧩 Objetos del mapa

El mapa se define con un archivo `.txt` donde cada carácter representa un bloque:

| Símbolo | Objeto |
|----------|--------|
| `#` | Pared |
| `.` | Espacio vacío |
| `P` | Posición inicial del jugador |
| `F` | Enemigo |
| `T` | Worker |
| `C` | Cofre |

Ejemplo de mapa (`maze.txt`):

```
####################
#P.....#......#....#
#..F...#..C...#..T.#
#......#......#....#
####################
```

---

## 🧠 Créditos

- 💻 **Programación:** Tú (Rust + Raylib)
- 🎨 **Inspiración visual:** *OFF* de Mortis Ghost
- 🔊 **Audio:** efectos y música ambient con [rodio](https://crates.io/crates/rodio)
- 🧱 **Engine base:** [raylib-rs](https://github.com/deltaphc/raylib-rs)

---

## ⚙️ Dependencias (Cargo.toml)

Asegúrate de incluir esto en tu `Cargo.toml`:

```toml
[dependencies]
raylib = "5.0"
rodio = "0.17"
```

---

## 💡 Notas técnicas

- Usa **renderizado por raycasting 3D** con sprites tipo billboard.  
- Los cofres tienen estado `opened` y no pueden volver a activarse.  
- “Joker Received” aparece 2 segundos en pantalla tras abrir un cofre.  
- Sistema de menú inspirado en el juego original OFF (2008).

---

## 📜 Licencia

Este proyecto es un **fan game sin fines comerciales**.  
El contenido original pertenece a **Mortis Ghost / Unproductive Fun Time**.  
Usa este código libremente para fines educativos o recreativos.

---

🧡 *“Purification in progress...”*

# Proyecto 3 - Simulador de Sistema Solar

Este es un proyecto para la clase de Gráficas por Computadora en la Universidad del Valle de Guatemala. Es una simulación de un sistema solar renderizado en 3D usando Rust y OpenGL.

## Video de Demostración

[![Video](https://img.youtube.com/vi/RQ98xZr_7Qs/maxresdefault.jpg)](https://youtu.be/RQ98xZr_7Qs)

## Características

- Renderizado de planetas y otros cuerpos celestes.
- Cámara libre para explorar el sistema solar.
- Minimapa para la navegación.
- Efecto de warp para viajes rápidos.
- Skybox con un fondo estelar.
- Shaders personalizados para los planetas y otros efectos.

## Cómo ejecutar el proyecto

1.  Clona este repositorio.
2.  Asegúrate de tener Rust y Cargo instalados.
3.  Ejecuta el siguiente comando en la raíz del proyecto:

    ```bash
    cargo run --release
    ```

## Controles

- **W, A, S, D:** Mover la cámara.
- **Mouse:** Rotar la cámara.
- **Scroll:** Zoom in/out.

## Estructura del Proyecto

El proyecto está organizado de la siguiente manera:

- `src/`: Contiene todo el código fuente en Rust.
  - `main.rs`: El punto de entrada de la aplicación.
  - `renderer.rs`: El motor de renderizado principal.
  - `solar_system.rs`: Lógica para la simulación del sistema solar.
  - `celestial_body.rs`: Define la estructura y comportamiento de los cuerpos celestes.
  - `shaders/`: Contiene los shaders de GLSL.
- `assets/`: Contiene los modelos 3D y otras texturas.
- `Cargo.toml`: El manifiesto del paquete de Rust.

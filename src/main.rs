mod framebuffer;
mod mesh;
mod renderer;
mod celestial_body;
mod solar_system;
mod camera;
mod shaders;

use framebuffer::{Color, Framebuffer};
use mesh::ObjMesh;
use renderer::Renderer;
use celestial_body::CelestialType;
use solar_system::SolarSystemBuilder;
use camera::SpaceshipCamera;
use shaders::*;

use nalgebra_glm::{Vec3, perspective};
use raylib::prelude::*;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    println!("=== Iniciando Sistema Solar ===");

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Sistema Solar - Software Renderer")
        .build();

    rl.set_target_fps(60);
    rl.disable_cursor();

    // ========== CARGAR GEOMETRÍA (TODO AL INICIO) ==========
    println!("Generando geometría...");
    
    // Crear múltiples niveles de detalle
    let sphere_mesh_high = ObjMesh::create_sphere(1.0, 64, 64);    // Alta calidad
    let sphere_mesh_medium = ObjMesh::create_sphere(1.0, 32, 32);  // Media calidad
    let sphere_mesh_low = ObjMesh::create_sphere(1.0, 16, 16);     // Baja calidad
    let ring_mesh = ObjMesh::create_ring(1.3, 2.0, 100);

    // Cargar modelo de nave
    println!("Cargando modelo de nave...");
    let ship_mesh = match ObjMesh::load_from_obj("assets/ship.obj") {
        Ok(mesh) => {
            println!("✓ ship.obj cargado exitosamente");
            Some(mesh)
        }
        Err(e) => {
            println!("⚠ No se pudo cargar ship.obj: {}", e);
            println!("  La nave no será visible");
            None
        }
    };

    // Cargar sphere.obj de alta calidad (opcional)
    let high_quality_sphere = match ObjMesh::load_from_obj("assets/sphere.obj") {
        Ok(mesh) => {
            println!("✓ sphere.obj cargado exitosamente");
            Some(mesh)
        }
        Err(e) => {
            println!("⚠ No se pudo cargar sphere.obj: {}", e);
            println!("  Usando esferas procedurales");
            None
        }
    };

    // ========== FUNCIÓN PARA SELECCIONAR LOD ==========
    // Esta función decide qué malla usar según la distancia
    let get_sphere_lod = |distance: f32, radius: f32| -> &ObjMesh {
        if distance < radius * 5.0 {
            // Muy cerca: usar la mejor calidad disponible
            high_quality_sphere.as_ref().unwrap_or(&sphere_mesh_high)
        } else if distance < radius * 20.0 {
            // Cerca: alta calidad
            &sphere_mesh_high
        } else if distance < radius * 100.0 {
            // Media distancia: calidad media
            &sphere_mesh_medium
        } else {
            // Lejos: baja calidad
            &sphere_mesh_low
        }
    };

    // ========== CREAR SISTEMA SOLAR ==========
    println!("Creando sistema solar...");
    let celestial_bodies = SolarSystemBuilder::build_realistic();

    // ========== CREAR CÁMARA/NAVE ==========
    let mut camera = SpaceshipCamera::new(Vec3::new(0.0, 500.0, 8000.0));

    // ========== RENDERER Y FRAMEBUFFER ==========
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let renderer = Renderer::new(WIDTH, HEIGHT);

    let initial_image = Image::gen_image_color(
        WIDTH as i32, 
        HEIGHT as i32, 
        Color::BLACK.to_raylib()
    );
    let mut texture = rl.load_texture_from_image(&thread, &initial_image).unwrap();

    // ========== VARIABLES DE CONTROL ==========
    let mut paused = false;
    let mut simulation_time = 0.0f32;
    let mut show_orbits = true;
    let mut show_menu = false;
    let time_scale = 0.001; // Velocidad base de simulación

    // ========== INSTRUCCIONES ==========
    println!("CONTROLES:");
    println!("  WASD + Q/E: Movimiento de nave");
    println!("  Mouse Derecho: Rotar cámara");
    println!("  SHIFT: Turbo (x3)");
    println!("  F: Warp mode (x50)");
    println!("  G: Hyper Warp (x500)");
    println!("  H: Ultra Warp (x5000)");
    println!("  +/-: Acelerar/Ralentizar tiempo");
    println!("  C: Toggle 1ra/3ra persona");
    println!("  Scroll: Zoom (3ra persona)");
    println!("  SPACE: Pausar");
    println!("  O: Toggle órbitas");
    println!("  TAB: Menú de teleportación");
    println!("  ESC: Salir\n");

    // ========== LOOP PRINCIPAL ==========
    while !rl.window_should_close() {
        // ===== CONTROL DE TIEMPO =====
        let mut current_time_scale = time_scale;
        
        if rl.is_key_down(KeyboardKey::KEY_KP_ADD) || rl.is_key_down(KeyboardKey::KEY_EQUAL) {
            current_time_scale *= 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_KP_SUBTRACT) || rl.is_key_down(KeyboardKey::KEY_MINUS) {
            current_time_scale *= 0.5;
        }

        if !paused {
            simulation_time += current_time_scale;
        }

        // ===== CONTROLES GLOBALES =====
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_O) {
            show_orbits = !show_orbits;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            show_menu = !show_menu;
            if show_menu {
                rl.enable_cursor();
            } else {
                rl.disable_cursor();
            }
        }

        // ===== MENÚ DE TELEPORTACIÓN =====
        if show_menu {
            if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
                camera.teleport_to(Vec3::zeros(), 500.0);
                show_menu = false;
                rl.disable_cursor();
            }
            
            for i in 1..=9 {
                let key = match i {
                    1 => KeyboardKey::KEY_ONE,
                    2 => KeyboardKey::KEY_TWO,
                    3 => KeyboardKey::KEY_THREE,
                    4 => KeyboardKey::KEY_FOUR,
                    5 => KeyboardKey::KEY_FIVE,
                    6 => KeyboardKey::KEY_SIX,
                    7 => KeyboardKey::KEY_SEVEN,
                    8 => KeyboardKey::KEY_EIGHT,
                    9 => KeyboardKey::KEY_NINE,
                    _ => KeyboardKey::KEY_ONE,
                };
                
                if rl.is_key_pressed(key) && i < celestial_bodies.len() {
                    let body = &celestial_bodies[i];
                    let parent_pos = match body.parent_index {
                        Some(p) => Some(celestial_bodies[p].get_world_position(simulation_time, None)),
                        None => None,
                    };
                    let target_pos = body.get_world_position(simulation_time, parent_pos);
                    camera.teleport_to(target_pos, body.radius * 3.0);
                    show_menu = false;
                    rl.disable_cursor();
                }
            }
        } else {
            camera.update(&rl);
        }

        // ===== MATRICES DE VISTA Y PROYECCIÓN =====
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = perspective(
            WIDTH as f32 / HEIGHT as f32,
            60.0_f32.to_radians(),
            0.1,
            500000.0,
        );

        // ===== LIMPIAR FRAMEBUFFER =====
        framebuffer.clear(Color::new(5, 5, 15));

        // TEST: Dibujar un cuadrado en el centro para verificar que el framebuffer funciona
        for y in 350..370 {
            for x in 630..650 {
                framebuffer.set_pixel(x, y, Color::new(255, 0, 0), 0.0);
            }
        }

        // ===== CALCULAR POSICIONES DE CUERPOS =====
        let mut world_positions = Vec::new();
        for body in celestial_bodies.iter() {
            let parent_pos = match body.parent_index {
                Some(p) => Some(world_positions[p]),
                None => None,
            };
            world_positions.push(body.get_world_position(simulation_time, parent_pos));
        }

        // ===== RENDERIZAR ÓRBITAS =====
        if show_orbits {
            for (i, body) in celestial_bodies.iter().enumerate() {
                if body.body_type != CelestialType::Star {
                    let orbit_points = body.get_orbit_points(100);
                    let parent_pos = match body.parent_index {
                        Some(p) => world_positions[p],
                        None => Vec3::zeros(),
                    };
                    
                    renderer.render_orbit(
                        &mut framebuffer,
                        &orbit_points,
                        parent_pos,
                        &view_matrix,
                        &projection_matrix,
                        Color::new(100, 100, 150),
                    );
                }
            }
        }

// ===== RENDERIZAR CUERPOS CELESTES CON LOD =====
let camera_pos = camera.get_camera_position();

println!("=== Frame {} ===", rl.get_frame_time());
println!("Total cuerpos: {}", celestial_bodies.len());
println!("Posición cámara: ({:.1}, {:.1}, {:.1})", camera_pos.x, camera_pos.y, camera_pos.z);

for (i, body) in celestial_bodies.iter().enumerate() {
    let world_pos = world_positions[i];
    let distance_from_camera = (world_pos - camera_pos).magnitude();
    
    println!("  {}: Dist={:.1}, Radius={:.1}", body.name, distance_from_camera, body.radius);
    
    // SIMPLE: Solo verificar distancia máxima
    if distance_from_camera > 500000.0 {
        println!("    -> Demasiado lejos");
        continue;
    }
    
    // SIMPLE: Solo evitar estar dentro
    if distance_from_camera < body.radius * 1.1 {
        println!("    -> Demasiado cerca");
        continue;
    }
    
    println!("    -> RENDERIZANDO");

    // ===== SELECCIONAR MALLA LOD =====
    let lod_mesh = get_sphere_lod(distance_from_camera, body.radius);

    // ===== CREAR MATRIZ DE MODELO =====
    let model_matrix = body.get_model_matrix(simulation_time, world_pos);

    // ===== SELECCIONAR SHADER =====
    let shader: Box<dyn PlanetShader> = match body.body_type {
        CelestialType::Star => Box::new(ClassicSunShader),
        CelestialType::Planet => {
            match body.name.as_str() {
                "Mercurio" => Box::new(MercuryShader),
                "Venus" => Box::new(VenusShader),
                "Tierra" => Box::new(EarthShader),
                "Marte" => Box::new(MarsShader),
                "Júpiter" => Box::new(JupiterShader),
                "Saturno" => Box::new(SaturnShader),
                "Urano" => Box::new(UranusShader),
                "Neptuno" => Box::new(NeptuneShader),
                _ => Box::new(RockyPlanet),
            }
        }
        CelestialType::Moon => Box::new(MoonShader),
        CelestialType::Ring => Box::new(RingShader),
    };

    // ===== RENDERIZAR MESH =====
    renderer.render_mesh(
        &mut framebuffer,
        lod_mesh,
        shader.as_ref(),
        &model_matrix,
        &view_matrix,
        &projection_matrix,
        simulation_time,
    );

    // ===== RENDERIZAR ANILLOS DE SATURNO =====
    if body.name == "Saturno" && distance_from_camera < body.radius * 50.0 {
        println!("    -> Renderizando anillos de Saturno");
        let ring_model = nalgebra_glm::rotate(
            &model_matrix,
            20.0_f32.to_radians(),
            &Vec3::new(1.0, 0.0, 0.3),
        );
        renderer.render_mesh(
            &mut framebuffer,
            &ring_mesh,
            &RingShader,
            &ring_model,
            &view_matrix,
            &projection_matrix,
            simulation_time,
        );
    }
}

        // ===== RENDERIZAR NAVE EN TERCERA PERSONA =====
        if camera.third_person {
            if let Some(ship) = &ship_mesh {
                let ship_model = camera.get_ship_model_matrix(0.5);
                
                renderer.render_mesh(
                    &mut framebuffer,
                    ship,
                    &SimpleMetallicShader,
                    &ship_model,
                    &view_matrix,
                    &projection_matrix,
                    simulation_time,
                );
            }
        }

        // ===== ACTUALIZAR TEXTURA =====
        texture.update_texture(framebuffer.as_bytes()).ok();

        // ===== DIBUJAR EN PANTALLA =====
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK.to_raylib());
        d.draw_texture(&texture, 0, 0, raylib::color::Color::WHITE);

        // ===== UI =====
        d.draw_fps(10, 10);
        
        // Posición
        d.draw_text(
            &format!("Pos: ({:.0}, {:.0}, {:.0})", 
                camera.position.x, camera.position.y, camera.position.z),
            10, 35, 16, raylib::color::Color::WHITE
        );
        
        // Velocidad
        let speed = camera.get_effective_speed();
        d.draw_text(
            &format!("Vel: {:.2} u/s", speed),
            10, 55, 16, raylib::color::Color::WHITE
        );

        // Modo de velocidad
        let speed_mode = camera.get_speed_mode();
        let mode_color = match speed_mode {
            "ULTRA WARP" => raylib::color::Color::RED,
            "HYPER WARP" => raylib::color::Color::MAGENTA,
            "WARP" => raylib::color::Color::CYAN,
            _ => raylib::color::Color::GREEN,
        };
        
        d.draw_text(
            &format!("[{}]", speed_mode),
            10, 75, 20, mode_color
        );

        // Velocidad de tiempo
        d.draw_text(
            &format!("Tiempo: {:.1}x", current_time_scale / time_scale),
            10, 100, 16, raylib::color::Color::SKYBLUE
        );

        // Cuerpo más cercano
        if let Some((idx, distance)) = camera.get_nearest_body_distance(&world_positions) {
            let body = &celestial_bodies[idx];
            d.draw_text(
                &format!("Cercano: {} ({:.0} u)", body.name, distance),
                10, 120, 16, raylib::color::Color::YELLOW
            );

            // Advertencia de proximidad
            if distance < body.radius * 3.0 {
                d.draw_text(
                    &format!("⚠ PROXIMIDAD: {}", body.name),
                    WIDTH as i32 / 2 - 100,
                    50,
                    24,
                    raylib::color::Color::RED
                );
            }

            // ETA
            if speed > 0.1 {
                let eta_seconds = distance / speed;
                let eta_text = if eta_seconds < 60.0 {
                    format!("ETA: {:.0}s", eta_seconds)
                } else if eta_seconds < 3600.0 {
                    format!("ETA: {:.1}min", eta_seconds / 60.0)
                } else {
                    format!("ETA: {:.1}h", eta_seconds / 3600.0)
                };
                
                d.draw_text(&eta_text, 10, 140, 16, raylib::color::Color::ORANGE);
            }
        }

        // Pausa
        if paused {
            d.draw_text("[PAUSADO]", 10, 160, 20, raylib::color::Color::RED);
        }

        // ===== MENÚ DE TELEPORTACIÓN =====
        if show_menu {
            let menu_x = WIDTH as i32 / 2 - 200;
            let menu_y = HEIGHT as i32 / 2 - 200;
            
            d.draw_rectangle(
                menu_x - 10, 
                menu_y - 10, 
                420, 
                420, 
                raylib::color::Color::new(0, 0, 0, 200)
            );
            
            d.draw_text(
                "MENU DE TELEPORTACIÓN", 
                menu_x, 
                menu_y, 
                20, 
                raylib::color::Color::YELLOW
            );
            
            for (i, body) in celestial_bodies.iter().enumerate() {
                let text = if i == 0 {
                    format!("0: {}", body.name)
                } else if i <= 9 {
                    format!("{}: {}", i, body.name)
                } else {
                    continue;
                };
                
                d.draw_text(
                    &text, 
                    menu_x, 
                    menu_y + 30 + i as i32 * 25, 
                    18, 
                    raylib::color::Color::WHITE
                );
            }
            
            d.draw_text(
                "Presiona TAB para cerrar", 
                menu_x, 
                menu_y + 380, 
                16, 
                raylib::color::Color::GRAY
            );
        }
    }

    println!("=== Cerrando aplicación ===");
}
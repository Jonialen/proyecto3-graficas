mod framebuffer;
mod mesh;
mod renderer;
mod celestial_body;
mod solar_system;
mod camera;
mod shaders;
mod trail;
mod ui;
mod skybox;
mod warp_effect;
mod minimap;

use warp_effect::WarpEffect;
use framebuffer::{Color, Framebuffer};
use mesh::ObjMesh;
use renderer::Renderer;
use celestial_body::CelestialType;
use solar_system::SolarSystemBuilder;
use camera::SpaceshipCamera;
use shaders::*;
use trail::ShipTrail;
use ui::GameUI;
use skybox::Skybox;
use minimap::Minimap;

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

    // =================== CARGA DE GEOMETRÍA ===================
    println!("Generando geometría...");
    
    let sphere_mesh_high = ObjMesh::create_sphere(1.0, 64, 64);
    let sphere_mesh_medium = ObjMesh::create_sphere(1.0, 32, 32);
    let sphere_mesh_low = ObjMesh::create_sphere(1.0, 16, 16);
    let ring_mesh = ObjMesh::create_ring(1.3, 2.0, 100);

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

    let get_sphere_lod = |distance: f32, radius: f32| -> &ObjMesh {
        if distance < radius * 5.0 {
            high_quality_sphere.as_ref().unwrap_or(&sphere_mesh_high)
        } else if distance < radius * 20.0 {
            &sphere_mesh_high
        } else if distance < radius * 100.0 {
            &sphere_mesh_medium
        } else {
            &sphere_mesh_low
        }
    };

    // =================== SISTEMA SOLAR ===================
    println!("Creando sistema solar...");
    let celestial_bodies = SolarSystemBuilder::build_realistic();
    println!("✓ Sistema solar creado con {} cuerpos", celestial_bodies.len());

    let mut camera = SpaceshipCamera::new(Vec3::new(0.0, 500.0, 8000.0));
    let mut warp_effect = WarpEffect::new();

    // =================== FRAMEBUFFER + TEXTURA ===================
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let renderer = Renderer::new(WIDTH, HEIGHT);

    let initial_image = Image::gen_image_color(
        WIDTH as i32, 
        HEIGHT as i32, 
        Color::BLACK.to_raylib()
    );
    let mut texture = rl.load_texture_from_image(&thread, &initial_image).unwrap();

    // =================== TRAIL, SKYBOX, FLAGS ===================
    println!("Inicializando sistemas visuales...");
    let mut ship_trail = ShipTrail::new(200);
    let skybox = Skybox::new(2000);

    let mut show_trail = true;
    let mut minimap = Minimap::new(200);
    let mut show_minimap = true;
    let mut show_info = true;

    println!("✓ Trail inicializado");
    println!("✓ Skybox generado");

    // =================== VARIABLES ===================
    let mut paused = false;
    let mut simulation_time = 0.0f32;
    let mut frame_time = 0.0f32;
    let mut show_orbits = true;
    let mut show_menu = false;
    let time_scale = 0.001;

    println!("=== Sistema iniciado correctamente ===\n");

    // =================== LOOP PRINCIPAL ===================
    // =================== LOOP PRINCIPAL ===================
    while !rl.window_should_close() {
        frame_time += rl.get_frame_time();

        // ------------ Control de tiempo ------------
        let mut current_time_scale = time_scale;

        if rl.is_key_down(KeyboardKey::KEY_KP_ADD) 
            || rl.is_key_down(KeyboardKey::KEY_EQUAL) 
        {
            current_time_scale *= 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_KP_SUBTRACT) 
            || rl.is_key_down(KeyboardKey::KEY_MINUS)
        {
            current_time_scale *= 0.5;
        }

        if !paused {
            simulation_time += current_time_scale;
        }

        // ------------ Calcular posiciones de cuerpos (MOVER AQUÍ) ------------
        let mut world_positions = Vec::new();
        for body in celestial_bodies.iter() {
            let parent_pos = body.parent_index.map(|p| world_positions[p]);
            world_positions.push(
                body.get_world_position(simulation_time, parent_pos)
            );
        }

        // ✅ NUEVO: Preparar datos de colisión (ANTES del manejo de input)
        let collision_data: Vec<(Vec3, f32)> = celestial_bodies
            .iter()
            .enumerate()
            .map(|(i, _body)| (world_positions[i], celestial_bodies[i].radius))
            .collect();

        let proximity_mode = camera.get_proximity_mode(&collision_data);

        // ------------ Entradas globales ------------
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_O) {
            show_orbits = !show_orbits;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            show_trail = !show_trail;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            show_minimap = !show_minimap;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_I) {
            show_info = !show_info;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            show_menu = !show_menu;
            if show_menu { rl.enable_cursor(); }
            else { rl.disable_cursor(); }
        }

        // ------------ Teleportación ------------
        if show_menu {
            if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
                let target_pos = Vec3::zeros();
                
                // ✅ Iniciar warp animado
                warp_effect.start_warp(camera.position, target_pos, 1.5);
                
                ship_trail.clear();
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
                    let _body = &celestial_bodies[i];
                    let target = world_positions[i];
                    
                    // ✅ Warp animado
                    warp_effect.start_warp(camera.position, target, 2.0);
                    
                    ship_trail.clear();
                    show_menu = false;
                    rl.disable_cursor();
                }
            }
        } 
        else {
            // ✅ Actualizar warp
            if let Some(warp_pos) = warp_effect.update(rl.get_frame_time()) {
                camera.position = warp_pos;
                camera.sync_smoothed_position(); // Usar método público
            }
            
            // Solo permitir control manual si no estamos en warp
            if !warp_effect.is_active() {
                camera.update(&rl);
                
                // ✅ Sistema de colisión
                camera.check_collisions(&collision_data);
                
                if show_trail && !paused {
                    ship_trail.update(camera.position, frame_time);
                }
            }
        }

        // ------------ Matrices ------------
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = perspective(
            WIDTH as f32 / HEIGHT as f32,
            60.0_f32.to_radians(), 
            50.0, 
            500000.0,
        );
        let projection_matrix_near = perspective(
            WIDTH as f32 / HEIGHT as f32,
            60.0_f32.to_radians(), 
            0.1, 
            1000.0,
        );

        // ------------ Limpiar framebuffer ------------
        framebuffer.clear(Color::new(5, 5, 15));

        // ------------ Skybox ------------
        skybox.render(
            &mut framebuffer,
            &view_matrix,
            &projection_matrix,
            WIDTH as f32,
            HEIGHT as f32,
        );

        // ------------ Órbitas ------------
        if show_orbits {
            for (_i, body) in celestial_bodies.iter().enumerate() {
                if body.body_type == CelestialType::Asteroid {
                    continue;
                }
                
                if body.body_type != CelestialType::Star {
                    let orbit_points = body.get_orbit_points(100);
                    let parent_pos = body.parent_index
                        .map(|p| world_positions[p])
                        .unwrap_or(Vec3::zeros());
                    
                    let orbit_color = match body.body_type {
                        CelestialType::Moon => Color::new(80, 80, 100),
                        _ => Color::new(100, 100, 150),
                    };
                    
                    renderer.render_orbit(
                        &mut framebuffer,
                        &orbit_points,
                        parent_pos,
                        &view_matrix,
                        &projection_matrix,
                        orbit_color,
                    );
                }
            }
        }

        // ------------ Render de cuerpos ------------
        let camera_pos = camera.get_camera_position();

        for (i, body) in celestial_bodies.iter().enumerate() {
            let world_pos = world_positions[i];
            let dist = (world_pos - camera_pos).magnitude();

            if dist < body.radius * 1.5 {
                continue;
            }

            if !renderer.is_in_frustum(&world_pos, body.radius, &view_matrix, &projection_matrix) {
                continue;
            }

            let lod_mesh = get_sphere_lod(dist, body.radius);
            let model_matrix = body.get_model_matrix(simulation_time, world_pos);

            let shader: Box<dyn PlanetShader> = match body.body_type {
                CelestialType::Star => Box::new(ClassicSunShader),
                CelestialType::Planet => match body.name.as_str() {
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
                CelestialType::Moon => Box::new(MoonShader),
                CelestialType::Ring => Box::new(RingShader),
                CelestialType::Asteroid => Box::new(AsteroidShader),
            };

            renderer.render_mesh(
                &mut framebuffer,
                lod_mesh,
                shader.as_ref(),
                &model_matrix,
                &view_matrix,
                &projection_matrix,
                simulation_time,
            );

            if body.name == "Saturno" && dist < body.radius * 50.0 {
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

        // ------------ Trail ------------
        if show_trail {
            ship_trail.render(
                &mut framebuffer,
                &renderer,
                &view_matrix,
                &projection_matrix,
            );
        }

        // ------------ Nave 3ra persona ------------
        if camera.third_person {
            if let Some(ship) = &ship_mesh {
                let ship_projection = perspective(
                    WIDTH as f32 / HEIGHT as f32,
                    60.0_f32.to_radians(),
                    0.001,
                    50.0,
                );

                let ship_scale = camera.get_ship_scale();
                let ship_model = camera.get_ship_model_matrix_fixed(ship_scale);

                // ✅ Seleccionar método de renderizado según proximidad
                match proximity_mode {
                    camera::ProximityMode::Critical => {
                        // MUY CERCA: Renderizar sin z-test (siempre visible)
                        renderer.render_mesh_overlay(
                            &mut framebuffer,
                            ship,
                            &SimpleMetallicShader,
                            &ship_model,
                            &view_matrix,
                            &ship_projection,
                            simulation_time,
                        );
                    }
                    camera::ProximityMode::Close => {
                        // CERCA: Bias muy agresivo
                        renderer.render_mesh_with_bias(
                            &mut framebuffer,
                            ship,
                            &SimpleMetallicShader,
                            &ship_model,
                            &view_matrix,
                            &ship_projection,
                            simulation_time,
                            -0.5, // Bias extremo
                        );
                    }
                    camera::ProximityMode::Normal => {
                        // LEJOS: Bias normal
                        renderer.render_mesh_with_bias(
                            &mut framebuffer,
                            ship,
                            &SimpleMetallicShader,
                            &ship_model,
                            &view_matrix,
                            &ship_projection,
                            simulation_time,
                            -0.05,
                        );
                    }
                }
            }
        }

        // ------------ Efecto de Warp (ANTES de actualizar textura) ------------
        warp_effect.render(&mut framebuffer);

        // ===== ACTUALIZAR TEXTURA (ANTES DE begin_drawing) =====
        texture.update_texture(framebuffer.as_bytes()).ok();

        if show_minimap {
            minimap.handle_input(&rl);
        }

        // ===== CALCULAR VARIABLES PARA UI =====
        let speed = camera.get_effective_speed();
        let speed_mode = camera.get_speed_mode();
        let mode_color = match speed_mode {
            "ULTRA WARP" => raylib::color::Color::RED,
            "HYPER WARP" => raylib::color::Color::MAGENTA,
            "WARP" => raylib::color::Color::CYAN,
            _ => raylib::color::Color::GREEN,
        };

        let nearest_body = camera.get_nearest_body_distance(&world_positions);

        // =====================================================================
        // =================== DIBUJAR EN PANTALLA =============================
        // =====================================================================

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK.to_raylib());
        d.draw_texture(&texture, 0, 0, raylib::color::Color::WHITE);

        d.draw_fps(10, 10);

        d.draw_text(
            &format!("Pos: ({:.0}, {:.0}, {:.0})",
                camera.position.x, camera.position.y, camera.position.z),
            10, 35, 16, raylib::color::Color::WHITE
        );

        d.draw_text(
            &format!("Vel: {:.2} u/s", speed),
            10, 55, 16, raylib::color::Color::WHITE
        );

        d.draw_text(
            &format!("[{}]", speed_mode),
            10, 75, 20, mode_color
        );

        d.draw_text(
            &format!("Tiempo: {:.1}x", current_time_scale / time_scale),
            10, 100, 16, raylib::color::Color::SKYBLUE
        );

        d.draw_text(
            "[ / ] Zoom | L Labels | K Dist | M Toggle",
            WIDTH as i32 - 250,
            HEIGHT as i32 - 25,
            12,
            raylib::color::Color::new(150, 150, 180, 200),
        );

        // ----- Advertencia de colisión -----
        if let Some((idx, distance, severity)) = camera.get_collision_warning(&collision_data) {
            let body = &celestial_bodies[idx];
            let color = match severity {
                "CRÍTICA" => raylib::color::Color::RED,
                "ALTA" => raylib::color::Color::ORANGE,
                _ => raylib::color::Color::YELLOW,
            };
            
            d.draw_text(
                &format!("⚠ COLISIÓN {} - {}", severity, body.name),
                WIDTH as i32 / 2 - 150,
                30,
                20,
                color
            );
            
            d.draw_text(
                &format!("Distancia: {:.0}u (Mín: {:.0}u)", 
                    distance, body.radius * 2.5),
                WIDTH as i32 / 2 - 150,
                55,
                16,
                color
            );
        }

        // ----- Info del cuerpo más cercano -----
        if let Some((idx, distance)) = nearest_body {
            let body = &celestial_bodies[idx];
            
            d.draw_text(
                &format!("Cercano: {} ({:.0} u)", body.name, distance),
                10, 120, 16, raylib::color::Color::YELLOW
            );

            if show_info && distance < 50000.0 {
                GameUI::draw_planet_info(&mut d, body, distance, speed);
            }

            if distance < body.radius * 3.0 {
                d.draw_text(
                    &format!("⚠ PROXIMIDAD: {}", body.name),
                    WIDTH as i32 / 2 - 100,
                    50,
                    24,
                    raylib::color::Color::RED
                );
            }

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

        // ----- Minimapa -----
        if show_minimap {      
            minimap.render(
                &mut d,
                WIDTH as i32,
                HEIGHT as i32,
                &world_positions,
                &celestial_bodies,
                &camera.position,
                &camera.forward,
                frame_time,
            );
        }

        // ----- Indicadores -----
        if paused {
            d.draw_text("[PAUSADO]", 10, 160, 20, raylib::color::Color::RED);
        }

        if show_trail {
            d.draw_text("TRAIL: ON", WIDTH as i32 - 120, 10, 14, raylib::color::Color::GREEN);
        }

        // ----- Menú de teleportación -----
        if show_menu {
            let menu_x = WIDTH as i32 / 2 - 200;
            let menu_y = HEIGHT as i32 / 2 - 250;

            d.draw_rectangle(menu_x - 10, menu_y - 10, 420, 520, raylib::color::Color::new(0,0,0,220));
            d.draw_rectangle_lines(menu_x - 10, menu_y - 10, 420, 520, raylib::color::Color::SKYBLUE);

            d.draw_text("MENU DE TELEPORTACIÓN", menu_x, menu_y, 20, raylib::color::Color::YELLOW);
            d.draw_line(menu_x, menu_y + 25, menu_x + 400, menu_y + 25, raylib::color::Color::SKYBLUE);

            let mut display_index = 0;

            for (i, body) in celestial_bodies.iter().enumerate() {
                if body.body_type == CelestialType::Asteroid {
                    continue;
                }

                let text = if i <= 9 {
                    format!("{}: {}", i, body.name)
                } else {
                    continue;
                };

                d.draw_text(&text, menu_x, menu_y + 40 + display_index * 25, 18, raylib::color::Color::WHITE);
                display_index += 1;
            }

            d.draw_text("Presiona TAB para cerrar", menu_x, menu_y + 480, 16, raylib::color::Color::GRAY);
        }

        // ----- Ayuda rápida -----
        let show_help = d.is_key_down(KeyboardKey::KEY_F1);

        if show_help {
            let help_x = WIDTH as i32 / 2 - 150;
            let help_y = 100;

            d.draw_rectangle(help_x - 10, help_y - 10, 320, 280, raylib::color::Color::new(0,0,0,200));

            d.draw_text("AYUDA RÁPIDA", help_x, help_y, 18, raylib::color::Color::YELLOW);
            d.draw_text("T - Toggle Trail", help_x, help_y + 30, 14, raylib::color::Color::WHITE);
            d.draw_text("M - Toggle Minimapa", help_x, help_y + 50, 14, raylib::color::Color::WHITE);
            d.draw_text("I - Toggle Info", help_x, help_y + 70, 14, raylib::color::Color::WHITE);
            d.draw_text("O - Toggle Órbitas", help_x, help_y + 90, 14, raylib::color::Color::WHITE);
            d.draw_text("C - Cambiar vista", help_x, help_y + 110, 14, raylib::color::Color::WHITE);
            d.draw_text("F/G/H - Modos Warp", help_x, help_y + 130, 14, raylib::color::Color::WHITE);
            d.draw_text("TAB - Teleportación", help_x, help_y + 150, 14, raylib::color::Color::WHITE);
            d.draw_text("SPACE - Pausar", help_x, help_y + 170, 14, raylib::color::Color::WHITE);

            d.draw_text("Mantén F1 para ver ayuda", help_x - 30, help_y + 220, 12, raylib::color::Color::GRAY);
        } else {
            d.draw_text("F1 - Ayuda", WIDTH as i32 - 100, HEIGHT as i32 - 25, 14, raylib::color::Color::GRAY);
        }

        // El mutable borrow (d) termina aquí
    }

    println!("\n=== Cerrando aplicación ===");
}

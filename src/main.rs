use macroquad::prelude::*;

// 1. Definições de Dados
struct Player {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
}

#[macroquad::main("Bullet Dodger")]
async fn main() {
    // 2. Inicialização (Antes do jogo começar)
    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: 30.0,
        speed: 300.0, // 300 pixels por segundo
    };

    let mut enemies: Vec<Enemy> = Vec::new(); // Lista vazia de inimigos
    let mut spawn_timer: f32 = 0.0; // Cronômetro para o spawn
    let mut game_over = false; // Começa como falso

    // 3. O Game Loop (Roda 60 vezes por segundo)
    loop {
        clear_background(DARKGRAY);
        // --- DETECÇÃO DE COLISÃO ---
        if !game_over { // Só checa colisão se o jogo ainda estiver rolando
            for enemy in enemies.iter() {
                // 1. Acha o ponto do quadrado mais próximo do centro do inimigo
                let closest_x = enemy.x.clamp(player.x, player.x + player.size);
                let closest_y = enemy.y.clamp(player.y, player.y + player.size);

                // 2. Calcula a distância entre esse ponto e o centro do inimigo
                let distance_x = enemy.x - closest_x;
                let distance_y = enemy.y - closest_y;
                
                // Teorema de Pitágoras: a² + b² = c² (pegamos a distância ao quadrado para poupar processamento)
                let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

                // 3. Verifica se a distância é menor que o raio do inimigo ao quadrado
                if distance_squared < (enemy.radius * enemy.radius) {
                    game_over = true; // Bateu!
                    break; // Sai do loop de verificação, não precisa checar os outros
                }
            }
        }

        // Se deu game over, o tempo "para" (delta_time vira zero)
        let delta_time = if game_over { 0.0 } else { get_frame_time() };

        // --- LÓGICA DO JOGADOR ---
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.x += player.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.x -= player.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            player.y += player.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            player.y -= player.speed * delta_time;
        }

        // Colisão com as bordas
        player.x = player.x.clamp(0.0, screen_width() - player.size);
        player.y = player.y.clamp(0.0, screen_height() - player.size);


        // --- LÓGICA DOS INIMIGOS ---
        spawn_timer -= delta_time;
        if spawn_timer <= 0.0 {
            let new_enemy = Enemy {
                x: rand::gen_range(0.0, screen_width()), 
                y: -20.0, 
                radius: 15.0,
                speed: rand::gen_range(150.0, 400.0), 
            };
            enemies.push(new_enemy); 
            spawn_timer = 0.3; 
        }

        for enemy in enemies.iter_mut() {
            enemy.y += enemy.speed * delta_time;
        }

        enemies.retain(|enemy| enemy.y < screen_height() + enemy.radius);


        // --- DESENHAR NA TELA ---
        
        // Desenha o jogador
        draw_rectangle(player.x, player.y, player.size, player.size, BLUE);

        // Desenha os inimigos
        for enemy in enemies.iter() {
            draw_circle(enemy.x, enemy.y, enemy.radius, RED);
        }
// --- TELA DE GAME OVER ---
        if game_over {
            let text = "GAME OVER!";
            let text_size = 50.0;
            // Mede o tamanho do texto para centralizar na tela
            let text_dims = measure_text(text, None, text_size as u16, 1.0);
            
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                screen_height() / 2.0,
                text_size,
                RED,
            );

            let restart_text = "Pressione ESPACO para reiniciar";
            draw_text(
                restart_text,
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 + 40.0,
                25.0,
                WHITE,
            );

            // Lógica para reiniciar o jogo
            if is_key_pressed(KeyCode::Space) {
                game_over = false;
                enemies.clear(); // Limpa todos os inimigos da tela
                player.x = screen_width() / 2.0; // Volta o player pro centro
                player.y = screen_height() / 2.0;
                spawn_timer = 0.0;
            }
        }
        next_frame().await;
    }
}
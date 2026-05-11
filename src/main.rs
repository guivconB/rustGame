use macroquad::prelude::*;

// --- ESTRUTURAS ---
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
    // --- INICIALIZAÇÃO ---
    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: 30.0,
        speed: 300.0,
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut spawn_timer: f32 = 0.0;
    let mut game_over = false;
    
    // Novas variáveis para Pontuação e Dificuldade
    let mut score: f32 = 0.0;

    // --- GAME LOOP ---
    loop {
        clear_background(DARKGRAY);

        // Se deu game over, o tempo congela (delta_time = 0)
        let delta_time = if game_over { 0.0 } else { get_frame_time() };

        if !game_over {
            // 1. LÓGICA DE PONTUAÇÃO E DIFICULDADE
            score += 15.0 * delta_time; // Ganha 15 pontos por segundo vivo
            
            // Dificuldade dinâmica baseada no Score:
            // O delay inicial é 0.3s. Vai caindo até o mínimo de 0.05s (insano!)
            let current_spawn_delay = (0.3 - (score / 1500.0)).max(0.05);
            
            // A velocidade dos inimigos também aumenta com o score
            let base_speed_increase = score; 

            // 2. MOVIMENTO DO JOGADOR
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

            // Impede o jogador de sair da tela
            player.x = player.x.clamp(0.0, screen_width() - player.size);
            player.y = player.y.clamp(0.0, screen_height() - player.size);

            // 3. SPAWN E MOVIMENTO DOS INIMIGOS
            spawn_timer -= delta_time;
            if spawn_timer <= 0.0 {
                let new_enemy = Enemy {
                    x: rand::gen_range(0.0, screen_width()), 
                    y: -20.0, 
                    radius: 15.0,
                    // Usa a dificuldade dinâmica na velocidade
                    speed: rand::gen_range(150.0 + base_speed_increase, 400.0 + base_speed_increase), 
                };
                enemies.push(new_enemy); 
                spawn_timer = current_spawn_delay; 
            }

            for enemy in enemies.iter_mut() {
                enemy.y += enemy.speed * delta_time;
            }

            // Limpa da memória os que saíram da tela
            enemies.retain(|enemy| enemy.y < screen_height() + enemy.radius);

            // 4. DETECÇÃO DE COLISÃO
            for enemy in enemies.iter() {
                let closest_x = enemy.x.clamp(player.x, player.x + player.size);
                let closest_y = enemy.y.clamp(player.y, player.y + player.size);

                let distance_x = enemy.x - closest_x;
                let distance_y = enemy.y - closest_y;
                let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

                if distance_squared < (enemy.radius * enemy.radius) {
                    game_over = true;
                    break;
                }
            }
        }

        // --- DESENHAR NA TELA ---
        
        // Desenha o jogador
        draw_rectangle(player.x, player.y, player.size, player.size, BLUE);

        // Desenha os inimigos
        for enemy in enemies.iter() {
            draw_circle(enemy.x, enemy.y, enemy.radius, RED);
        }

        // Desenha o Placar (Convertendo o float para inteiro para ficar bonito)
        draw_text(&format!("PONTOS: {}", score as i32), 20.0, 40.0, 40.0, WHITE);

        // --- TELA DE GAME OVER ---
        if game_over {
            let text = "GAME OVER!";
            let text_size = 70.0;
            let text_dims = measure_text(text, None, text_size as u16, 1.0);
            
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                screen_height() / 2.0 - 50.0,
                text_size,
                RED,
            );

            let restart_text = "Pressione ESPACO para reiniciar";
            let restart_dims = measure_text(restart_text, None, 30, 1.0);
            draw_text(
                restart_text,
                screen_width() / 2.0 - restart_dims.width / 2.0,
                screen_height() / 2.0 + 20.0,
                30.0,
                WHITE,
            );

            // Resetando o jogo ao apertar Espaço
            if is_key_pressed(KeyCode::Space) {
                game_over = false;
                enemies.clear(); // Apaga as balas da tela
                player.x = screen_width() / 2.0; // Devolve pro centro
                player.y = screen_height() / 2.0;
                spawn_timer = 0.0;
                score = 0.0; // Zera os pontos
            }
        }

        next_frame().await;
    }
}
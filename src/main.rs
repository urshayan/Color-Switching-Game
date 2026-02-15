
use raylib::prelude::*;
use rand::Rng;

enum GameState{
    MENU,
    PLAYING,
    GAMEOVER,
}




// enumy ---  implemented in structs!
     struct Enemy{
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        speed: f32,
        color_index: usize,

     }

fn main() {
    
    // Window Setup 

    let  mut state = GameState::MENU;

    let (mut rl, thread) = raylib::init()
        .size(800,400)
        .title("Raylib in Rust")
        .build();

    rl.set_target_fps(60);


    let mut enemies: Vec<Enemy> = Vec::new();
    let mut spawn_timer = 0.0;
    let spawn_delay = 1.2; //in sec

    let mut score: i32 = 0;

   // Player Data -- intentionallu not a struct! 
    let player_x = 200.0;
    let player_y = 275.0;
    let radius = 30.0;

    let mut color_index = 0;

    let colors = [Color::RED , Color::GREEN, Color::BLUE];

    while !rl.window_should_close(){
       
        // ***************************************************************************
        //                        MENU 
        //**************************************************************************
        while let GameState::MENU = state {
            
        let x = rl.get_time();
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER){
            state = GameState::PLAYING;
            break;
        }

        let mut b = rl.begin_drawing(&thread);
        
        for i in 0..10 {
            let x = (i as f32 * 80.0 + x as f32 * 50.0) % 800.0;
            let y = (i as f32 * 40.0 + x as f32 * 30.0) % 400.0;
            b.draw_circle(x as i32, y as i32, 15.0, Color::GRAY);
        }
            b.draw_text("COLOR SWITCH GAME", 150, 50, 40, Color::RED);
            b.draw_text("Press ENTER to Start", 250, 150, 20, Color::LIGHTGRAY);
            b.draw_text("Z = RED, X = GREEN, C = BLUE", 220, 200, 20, Color::GREEN);
    
        }

    //**************************************************************
    //              GAME OVER!
    //***************************************************************
    while let GameState::GAMEOVER = state {
        let x = rl.get_time();
        let mut c = rl.begin_drawing(&thread);
        if c.is_key_pressed(KeyboardKey::KEY_R){
            state = GameState::PLAYING; 
            enemies.clear();        // remove all old enemies
            spawn_timer = 0.0;      // reset spawn timer
            color_index = 0;  
            score = 0;
            break;
        }
        if c.is_key_pressed(KeyboardKey::KEY_M){
            state = GameState::MENU;
            enemies.clear();
            spawn_timer = 0.0;
            color_index = 0;
            score = 0;
            break;
        }
        for i in 0..10 {
            let x = (i as f32 * 80.0 + x as f32 * 60.0) % 800.0;
            let y = ((i as f32 * 50.0 + x as f32 * 100.0) % 400.0) - 50.0;
            c.draw_rectangle(x as i32, y as i32, 50, 50, Color::RED.fade(0.5));
        }

// Game over text
        c.draw_text("GAME OVER", 220, 100, 50, Color::RED);
        c.draw_text(&format!("Score: {}", score), 650, 20, 30, Color::WHITE);
        c.draw_text("Press R to Restart", 250, 200, 25, Color::LIGHTGRAY);
        c.draw_text("Press M  to MENU", 250, 230, 25, Color::LIGHTGRAY);
    }    



//**************************************************************************
//Playing!
//********************************************************************************
        
    while let GameState::PLAYING = state {
        spawn_timer += rl.get_frame_time();

        if spawn_timer >= spawn_delay{
            spawn_timer = 0.0;

            let mut rng = rand::thread_rng();

            let enemy = Enemy{
                x: 810.0,
                y: 250.0,
                width: 50.0,
                height: 50.0,
                speed: 200.0,
                color_index: rng.gen_range(0..3),

            };
            enemies.push(enemy);
        }

        // Updates are here bih!

            for enemy in enemies.iter_mut(){
                enemy.x -= enemy.speed * rl.get_frame_time();
        }
            enemies.retain(|e| e.x + e.width > 0.0);







        // input
        if rl.is_key_pressed(KeyboardKey::KEY_Z){
            color_index = 0;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_X){
            color_index = 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_C){
            color_index = 2;
        }
         


        // drawing section 
        let mut d = rl.begin_drawing(&thread);


        d.clear_background(Color::PINK);
        d.draw_text(&format!("Score: {}", score), 600, 20, 30, Color::WHITE);
        d.draw_line(0, 300, 800, 300, Color::DARKGRAY);
        d.draw_circle(player_x as i32, (player_y + 5.0) as i32, radius, Color::BLACK.fade(0.3));
        d.draw_circle(
            player_x as i32,
            player_y as i32,
            radius,
            colors[color_index],
            );

        for enemy in &enemies{

            d.draw_rectangle(
                enemy.x as i32,
                enemy.y as i32,
                enemy.width as i32,
                enemy.height as i32,
                colors[enemy.color_index],
                );

            let rect = Rectangle{
                x: enemy.x,
                y: enemy.y,
                width: enemy.width,
                height: enemy.height,
            };

            let player_pos = Vector2::new(player_x, player_y); // vec2 of the cirlce => player!
            // checking collison between cirle and rectangle 
            if rect.check_collision_circle_rec(player_pos, radius){
                //println!("Collision!");

                if color_index == enemy.color_index{
                    score += 1;
                    //println!("Same Color!");
                }else {
                    //println!("NOt same Color!");
                    spawn_timer = 0.0;
                    state = GameState::GAMEOVER;
                }

            }



        }

    } // end of playing game state!
    
    } // end while loop 
    
}

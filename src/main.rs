use macroquad::{prelude::*, rand::gen_range};

fn conf() -> Conf {
    Conf {
        window_title: String::from("Ciuuulinder"),
        window_height: 1200,
        window_width: 1200,
        fullscreen: false,
        // you can add other option too or use default by
        ..Default::default()
    }
}



#[macroquad::main(conf)]
async fn main() {
    // Initial game variables
    let cross_road: Texture2D = load_texture("assets/cross-road.png").await.unwrap();

    // GAME LOOP
    loop {

        // 1. PROCESS INPUT
        // Handles any user input that
        // has happened since the last call


        // 2. UPDATE THE STAGE
        // Advances the game simulation one step
        // It runs the AI and game mechanics




        // 3. RENDER / DRAW
        // Draws the game on the screen
        draw_texture(&cross_road, 0., 0., WHITE);

        next_frame().await;
    }
}
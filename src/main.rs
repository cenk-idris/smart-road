use macroquad::{prelude::*, rand::gen_range};
use macroquad::input::KeyCode::Right;
const CAR_SIZE: Vec2 = vec2(43., 33.);
const RADAR_SIZE: Vec2 = vec2(43., 33.);



fn conf() -> Conf {
    Conf {
        window_title: String::from("Smart Road"),
        window_height: 1200,
        window_width: 1200,
        fullscreen: false,
        // you can add other option too or use default by
        ..Default::default()
    }
}

struct Car {
    car_rect: Rect,
    current_direction: String,
    current_speed: f32,
    randomized_initial_speed: f32,
    radar: Rect

}

impl Car {
    fn new() -> Self {
        let behavior_code_list = ["RD", "RL", "RU"];
        let randomized_behavior_code = gen_range(0, 3);
        let spawning = match behavior_code_list[randomized_behavior_code] {
            "RU" => vec2(1050., 495.),
            "RL" => vec2(1050., 535.),
            "RD" => vec2(1050., 574.),
            _ => panic!("Unexpected lane"),
        };

        Car {
            car_rect: Rect::new(spawning.x, spawning.y, CAR_SIZE.x, CAR_SIZE.y),
            radar: Rect::new(spawning.x - RADAR_SIZE.x, spawning.y, RADAR_SIZE.x, RADAR_SIZE.y),
            current_direction: "West".to_string(),
            randomized_initial_speed: gen_range(1., 2.),
            current_speed: 0.,
        }
    }

    fn spawn_if_can(self, cars_ref: &mut Vec<Car>) {
        if !cars_ref
                .iter_mut()
                .any(|other_car| self.car_rect.intersect(other_car.car_rect).is_some()) {
            cars_ref.push(self)
        }
    }

    fn move_one_step(&mut self) {
        match &*self.current_direction {
            "West" => self.car_rect.x -= self.randomized_initial_speed,
            "North" => self.car_rect.y -= self.randomized_initial_speed,
            _ => {}
        };
    }

    fn update_radar(&mut self) {

    }

    fn draw_all_components(&self) {

        // Draw Radar Rect
        draw_rectangle(
            self.radar.x,
            self.radar.y,
            RADAR_SIZE.x,
            RADAR_SIZE.y,
            Color::new(1.0, 0.0, 0.0, 0.1)
        );

        // Draw Car Rect
        draw_rectangle(
            self.car_rect.x,
            self.car_rect.y,
            CAR_SIZE.x,
            CAR_SIZE.y,
            Color::new(0.0, 1.0, 0.0, 0.3)
        );

    }

}



#[macroquad::main(conf)]
async fn main() {
    // Initial game variables
    let cross_road: Texture2D = load_texture("assets/cross-road.png").await.unwrap();
    let mut cars: Vec<Car> = Vec::new();

    // GAME LOOP
    loop {

        // 1. PROCESS INPUT
        // Handles any user input that
        // has happened since the last call
        if is_key_pressed(Right) {
            let possible_new_car = Car::new();
            possible_new_car.spawn_if_can(&mut cars);
        }


        // 2. UPDATE THE STAGE
        // Advances the game simulation one step
        // It runs the AI and game mechanics

        // a method call, moves the cars one step based on their direction
        cars.iter_mut().for_each(|car| car.move_one_step());
        // a method call to update radar positions after moving the car





        // 3. RENDER / DRAW
        // Draws the game on the screen

        // Draw the cross roads aka the background
        draw_texture(&cross_road, 0., 0., WHITE);

        //Draw the car_rect
        cars.iter().for_each(|car| car.draw_all_components() );

        next_frame().await;
    }
}
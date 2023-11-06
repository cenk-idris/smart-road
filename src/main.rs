use macroquad::{prelude::*, rand::gen_range};
use macroquad::input::KeyCode::Right;
use uuid::Uuid;

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

#[derive(Clone)]
struct Car {
    uuid: Uuid,
    car_rect: Rect,
    current_direction: String,
    current_speed: f32,
    randomized_initial_speed: f32,
    radar: Rect,
    has_turned: bool,
    behavior_code: String,
    waiting_flag: bool,

}

impl Car {
    fn new() -> Self {

        let random_speed = gen_range(1., 2.);
        let behavior_code_list = ["RD", "RL", "RU"];
        let randomized_behavior_code = gen_range(0, 3);
        let spawning = match behavior_code_list[randomized_behavior_code] {
            "RU" => vec2(1050., 495.),
            "RL" => vec2(1050., 535.),
            "RD" => vec2(1050., 574.),
            _ => panic!("Unexpected lane"),
        };

        Car {
            uuid: Uuid::new_v4(),
            car_rect: Rect::new(spawning.x, spawning.y, CAR_SIZE.x, CAR_SIZE.y),
            radar: Rect::new(spawning.x - RADAR_SIZE.x, spawning.y, RADAR_SIZE.x, RADAR_SIZE.y),
            current_direction: "West".to_string(),
            randomized_initial_speed: random_speed,
            current_speed: random_speed,
            has_turned: false,
            behavior_code: behavior_code_list[randomized_behavior_code].to_string(),
            waiting_flag: false,

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
            "West" => self.car_rect.x -= self.current_speed,
            "North" => self.car_rect.y -= self.current_speed,
            _ => {}
        };
    }

    fn update_radar(&mut self, car_index: usize, temp_cars: &Vec<Car>) {
        match &*self.current_direction {
            "West" => {
                // Update radar rectangle
                self.radar.x = self.car_rect.x - RADAR_SIZE.x;
                // Reposition the radar when intersection occur
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index && self.radar.intersect(other_car.car_rect).is_some() {

                        self.radar.x = other_car.car_rect.x + other_car.car_rect.w;
                    }
                    // Update radar width
                    self.radar.w = vec2(self.radar.x, self.radar.y).distance(vec2(self.car_rect.x, self.car_rect.y)).min(43.);
                }
            }
            "North" => {
                // Update radar rectangle
                (self.radar.x, self.radar.y) = (self.car_rect.x, self.car_rect.y - RADAR_SIZE.x);
                //Reposition the radar when intersection occur
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index && self.radar.intersect(other_car.car_rect).is_some() {

                        self.radar.y = other_car.car_rect.y + other_car.car_rect.h;
                    }
                    // Update radar width
                    self.radar.h = vec2(self.radar.x, self.radar.y).distance(vec2(self.car_rect.x, self.car_rect.y)).min(43.);
                    self.radar.w = 33.;
                }
            }
            _ => {}
        }
    }


    fn turn_right(&mut self, temp_cars: &Vec<Car>) {
        if self.has_turned == false && self.behavior_code == "RU" && self.car_rect.x <= 683. {
            self.waiting_flag = true;
            let temp_rect = Rect::new(
                683.,
                self.car_rect.y - (self.car_rect.w - self.car_rect.h).abs(),
                self.car_rect.h,
                self.car_rect.w
            );
            println!("{:?}", self.car_rect);
            self.car_rect = temp_rect;
            self.waiting_flag = false;
            self.current_direction = "North".to_string();
            self.has_turned = true;
            println!("{:?}", self.car_rect);

        }
    }

    fn draw_all_components(&self) {

        // Draw Radar Rect
        draw_rectangle(
            self.radar.x,
            self.radar.y,
            self.radar.w,
            self.radar.h,
            Color::new(1.0, 0.0, 0.0, 0.1)
        );

        // Draw Car Rect
        draw_rectangle(
            self.car_rect.x,
            self.car_rect.y,
            self.car_rect.w,
            self.car_rect.h,
            Color::new(0.0, 1.0, 0.0, 0.3)
        );

        // Draw Car image top of rect
        //draw_text_ex()

    }

}




#[macroquad::main(conf)]
async fn main() {
    // Initial game variables
    let id_counter = 0;
    let mut is_paused = false;
    let cross_road: Texture2D = load_texture("assets/cross-road.png").await.unwrap();
    let mut cars: Vec<Car> = Vec::new();

    // GAME LOOP

    loop {
        if is_key_pressed(KeyCode::P) {
            is_paused = !is_paused;
        }

        if is_paused {
            draw_text("Game Paused - Press P to continue", 350., 600., 40.0, WHITE);
        } else {

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
            cars
                .iter_mut()
                .filter(|car| !car.waiting_flag)
                .for_each(|car| car.move_one_step());

            // a method call to update radar positions after moving the car

            let temp_cars = cars.clone();
            for (car_index, car) in cars.iter_mut().enumerate() {
                car.update_radar(car_index, &temp_cars);
            }

            for car in &mut cars {
                match car.radar.w {
                    radar_width if radar_width <= 4. => car.current_speed = 0.,
                    radar_width if radar_width <= 10. => {
                        car.current_speed = car.randomized_initial_speed * 0.25;
                    }
                    radar_width if radar_width <= 20. => {
                        car.current_speed = car.randomized_initial_speed * 0.5;
                    }
                    radar_width if radar_width <= 39. => {
                        car.current_speed = car.randomized_initial_speed * 0.75
                    }
                    _ => car.current_speed = car.randomized_initial_speed,
                }
            }

            let temp_cars = cars.clone();
            cars.iter_mut().for_each(|car| car.turn_right(&temp_cars));





            // 3. RENDER / DRAW
            // Draws the game on the screen

            // Draw the cross roads aka the background
            draw_texture(&cross_road, 0., 0., WHITE);

            //Draw the car_rect
            cars.iter().for_each(|car| car.draw_all_components() );
        }


        next_frame().await;
    }
}
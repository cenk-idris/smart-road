use macroquad::input::KeyCode::{Down, Left, Right, Up};
use macroquad::{prelude::*, rand::gen_range};
use std::default::Default;
mod stats;
use stats::*;
mod car;
use car::*;

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

#[macroquad::main(conf)]
async fn main() {
    // Initial game variables
    let mut statistics: Stats = Stats {
        total_cars: 0,
        best_time: 999999999.,
        worst_time: 0.,
        best_velocity: 0.,
        worst_velocity: 999999999.,
        close_calls: 0,
        collisions: 0,
    };

    let mut random_flag: bool = false;
    let mut is_escaped: bool = false;
    let mut is_exit: bool = false;
    let mut is_paused = false;
    let mut is_debug_mode = false;
    let cross_road: Texture2D = load_texture("assets/cross-road.png").await.unwrap();
    let car_texture: Texture2D = load_texture("assets/car.png").await.unwrap();
    let mut cars: Vec<Car> = Vec::new();
    let core_intersection = Rect::new(503., 520., 180., 180.);

    // GAME LOOP

    loop {
        if is_key_pressed(KeyCode::Escape) {
            if is_exit {
                std::process::exit(0);
            } else {
                is_escaped = true;
                is_exit = true;
            }
        }
        if is_key_pressed(KeyCode::P) {
            is_paused = !is_paused;
        }
        if is_key_pressed(KeyCode::D) {
            is_debug_mode = !is_debug_mode;
        }
        if is_escaped {
            statistics.draw_endgame();
        } else if is_paused {
            // 3. RENDER / DRAW
            // Draws the game on the screen

            // Draw the cross roads aka the background
            draw_texture(&cross_road, 0., 0., WHITE);
            if is_debug_mode {
                draw_rectangle(
                    core_intersection.x,
                    core_intersection.y,
                    core_intersection.w,
                    core_intersection.h,
                    Color::new(0.5, 0.5, 0., 0.1),
                );
            }

            //Draw the car_rect
            cars.iter()
                .for_each(|car| car.draw_all_components(&car_texture, is_debug_mode));
            // Draw PAUSED TEXT
            draw_text("Press P to continue", 430., 600., 40., BLACK)
        } else {
            // 1. PROCESS INPUT
            // Handles any user input that
            // has happened since the last call

            if is_key_pressed(Left) {
                Car::spawn_if_can(&mut cars, vec!["RU", "RL", "RD"][gen_range(0, 3)], "West");
            }
            if is_key_pressed(Up) {
                Car::spawn_if_can(&mut cars, vec!["DU", "DL", "DR"][gen_range(0, 3)], "North");
            }
            if is_key_pressed(Down) {
                Car::spawn_if_can(&mut cars, vec!["UL", "UD", "UR"][gen_range(0, 3)], "South");
            }
            if is_key_pressed(Right) {
                Car::spawn_if_can(&mut cars, vec!["LU", "LR", "LD"][gen_range(0, 3)], "East");
            }
            if is_key_pressed(KeyCode::R) {
                random_flag = !random_flag;
            }
            if random_flag {
                let random_direction = vec!["West", "North", "South", "East"][gen_range(0, 4)];
                match random_direction {
                    "West" => {
                        Car::spawn_if_can(
                            &mut cars,
                            vec!["RU", "RL", "RD"][gen_range(0, 3)],
                            random_direction,
                        );
                    }
                    "North" => {
                        Car::spawn_if_can(
                            &mut cars,
                            vec!["DU", "DL", "DR"][gen_range(0, 3)],
                            random_direction,
                        );
                    }
                    "South" => {
                        Car::spawn_if_can(
                            &mut cars,
                            vec!["UL", "UD", "UR"][gen_range(0, 3)],
                            random_direction,
                        );
                    }
                    "East" => {
                        Car::spawn_if_can(
                            &mut cars,
                            vec!["LU", "LR", "LD"][gen_range(0, 3)],
                            random_direction,
                        );
                    }
                    _ => {}
                }
            }

            // 2. UPDATE THE STAGE
            // Advances the game simulation one step
            // It runs the AI and game mechanics
            cars.retain(|car| {
                if &*car.current_direction == "West" && car.car_rect.x < 100. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    if car.close_calls == 1 {
                        statistics.close_calls += 1;
                    }
                    if car.collisions {
                        statistics.collisions += 1;
                    }
                    false
                } else if &*car.current_direction == "North" && car.car_rect.y < 100. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    if car.close_calls == 1 {
                        statistics.close_calls += 1;
                    }
                    if car.collisions {
                        statistics.collisions += 1;
                    }
                    false
                } else if &*car.current_direction == "South" && car.car_rect.y > 1050. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    if car.close_calls == 1 {
                        statistics.close_calls += 1;
                    }
                    if car.collisions {
                        statistics.collisions += 1;
                    }
                    false
                } else if &*car.current_direction == "East"
                    && car.car_rect.x + car.car_size.long_edge > 1100.
                {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    if car.close_calls == 1 {
                        statistics.close_calls += 1;
                    }
                    if car.collisions {
                        statistics.collisions += 1;
                    }
                    false
                } else {
                    true
                }
            });

            let temp_cars = cars.clone();
            cars.iter_mut()
                .for_each(|car| car.communicate_with_intersection(&temp_cars, &core_intersection));

            cars.iter_mut().for_each(|car| car.adjust_current_speed());

            // a method call, moves the cars one step based on their direction
            let mut temp_cars = cars.clone();
            cars.iter_mut()
                .filter(|car| !car.waiting_flag)
                .for_each(|car| car.move_one_step_if_no_collide(&mut temp_cars));

            // a method call to update radar positions after moving the car

            let temp_cars = cars.clone();
            for (car_index, car) in cars.iter_mut().enumerate() {
                car.update_radar(car_index, &temp_cars);
            }

            let temp_cars = cars.clone();
            cars.iter_mut().for_each(|car| car.turn_if_can(&temp_cars));

            // check collisions
            let collision_results: Vec<bool> = cars
                .iter()
                .map(|car| {
                    let other_cars: Vec<_> = cars
                        .iter()
                        .filter(|other_car| other_car.uuid != car.uuid)
                        .collect();
                    other_cars
                        .iter()
                        .any(|other_car| car.collides_with(other_car))
                })
                .collect();

            for (car, &collided) in cars.iter_mut().zip(collision_results.iter()) {
                if collided {
                    car.collisions = true;
                }
            }

            // 3. RENDER / DRAW
            // Draws the game on the screen

            // Draw the cross roads aka the background
            draw_texture(&cross_road, 0., 0., WHITE);
            if is_debug_mode {
                draw_rectangle(
                    core_intersection.x,
                    core_intersection.y,
                    core_intersection.w,
                    core_intersection.h,
                    Color::new(0.5, 0.5, 0., 0.1),
                );
            }

            //Draw the car_rect
            cars.iter()
                .for_each(|car| car.draw_all_components(&car_texture, is_debug_mode));

            statistics.draw_ingame();
        }

        next_frame().await;
    }
}

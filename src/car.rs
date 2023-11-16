use crate::stats::*;
use macroquad::{prelude::*, rand::gen_range};
use std::time::Instant;
use uuid::Uuid;

pub const CAR_SIZE: Vec2 = vec2(43., 33.);
pub const RADAR_SIZE: Vec2 = vec2(43., 33.);
#[derive(Clone, Debug, PartialEq)]
pub struct Car {
    pub uuid: Uuid,
    pub spawn_point: Vec2,
    pub lifetime: Instant,
    pub car_rect: Rect,
    pub current_direction: String,
    pub current_speed: f32,
    pub randomized_initial_speed: f32,
    pub radar: Rect,
    pub proximity: f32,
    pub has_turned: bool,
    pub behavior_code: String,
    pub waiting_flag: bool,
    pub car_size: Dimensions,
    pub radar_size: Dimensions,
    pub dest_point: Vec2,
    pub close_calls: u8,
    pub collisions: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions {
    pub long_edge: f32,
    pub short_edge: f32,
    pub delta_edge: f32,
}

impl Car {
    pub fn new(randomized_behavior: &str, initial_direction: &str) -> Self {
        let random_speed = gen_range(0.8, 2.);
        let spawning = match randomized_behavior {
            "RU" => vec2(1050., 495.),
            "RL" => vec2(1050., 535.),
            "RD" => vec2(1050., 574.),
            "DU" => vec2(643., 1050.),
            "DL" => vec2(603., 1050.),
            "DR" => vec2(683., 1050.),
            "LU" => vec2(150., 617.),
            "LR" => vec2(150., 655.),
            "LD" => vec2(150., 695.),
            "UD" => vec2(516., 100.),
            "UR" => vec2(558., 100.),
            "UL" => vec2(477., 100.),
            _ => panic!("Unexpected lane"),
        };

        Car {
            uuid: Uuid::new_v4(),
            lifetime: Instant::now(),
            spawn_point: spawning,
            car_rect: if initial_direction == "West" || initial_direction == "East" {
                Rect::new(spawning.x, spawning.y, CAR_SIZE.x, CAR_SIZE.y)
            } else {
                Rect::new(spawning.x, spawning.y, CAR_SIZE.y, CAR_SIZE.x)
            },
            radar: Rect::new(
                spawning.x - RADAR_SIZE.x,
                spawning.y,
                RADAR_SIZE.x,
                RADAR_SIZE.y,
            ),
            proximity: RADAR_SIZE.x,
            current_direction: initial_direction.to_string(),
            randomized_initial_speed: random_speed,
            current_speed: random_speed,
            has_turned: false,
            behavior_code: randomized_behavior.to_string(),
            waiting_flag: false,

            car_size: Dimensions {
                long_edge: 43.,
                short_edge: 33.,
                delta_edge: CAR_SIZE.x - CAR_SIZE.y,
            },
            radar_size: Dimensions {
                long_edge: 43.,
                short_edge: 33.,
                delta_edge: CAR_SIZE.x - CAR_SIZE.y,
            },
            dest_point: match randomized_behavior {
                "RU" => vec2(683., 100.),
                "RL" => vec2(100., 535.),
                "RD" => vec2(555., 1050.),
                "DU" => vec2(643., 100.),
                "DL" => vec2(100., 574.),
                "DR" => vec2(1057., 695.),
                "LU" => vec2(593., 100.),
                "LR" => vec2(1057., 655.),
                "LD" => vec2(567., 1050.),
                "UD" => vec2(516., 1050.),
                "UR" => vec2(1057., 607.),
                "UL" => vec2(100., 485.),
                _ => panic!("Unexpected lane"),
            },
            close_calls: 0,
            collisions: false,
        }
    }

    pub fn collides_with(&self, other: &Car) -> bool {
        self.car_rect.intersect(other.car_rect).is_some()
    }

    pub fn spawn_if_can(
        cars_ref: &mut Vec<Car>,
        randomized_behavior: &str,
        initial_direction: &str,
    ) {
        let possible_new_car = Car::new(randomized_behavior, initial_direction);
        if !cars_ref.iter_mut().any(|other_car| {
            possible_new_car
                .car_rect
                .intersect(other_car.car_rect)
                .is_some()
        }) && cars_ref.len() < 20
        {
            cars_ref.push(possible_new_car)
        }
    }

    pub fn check_for_best_or_worst_time(&self, statistics: &mut Stats) {
        let temp_time = self.lifetime.elapsed().as_secs_f32();
        if temp_time < statistics.best_time {
            statistics.best_time = temp_time;
        }
        if temp_time > statistics.worst_time {
            statistics.worst_time = temp_time;
        }
        let temp_velocity = self.spawn_point.distance(self.dest_point) / temp_time;
        if temp_velocity > statistics.best_velocity {
            statistics.best_velocity = temp_velocity;
        }
        if temp_velocity < statistics.worst_velocity {
            statistics.worst_velocity = temp_velocity;
        }
    }

    pub fn communicate_with_intersection(&mut self, cars_ref: &Vec<Car>, core_intersection: &Rect) {
        let mut temp_cars = cars_ref.clone();
        temp_cars.retain(|car| car.uuid != self.uuid);
        if self.behavior_code == "LR"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "LR" || car.behavior_code == "DL")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "LU"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                car.behavior_code == "LU" && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "RD"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                car.behavior_code == "RD" && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "RL"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                car.behavior_code == "RL" && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }

        if self.behavior_code == "UR"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "UR" || car.behavior_code == "RL")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "UD"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "UD" || car.behavior_code == "RL")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }

        if self.behavior_code == "DL"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "DL"
                    || car.behavior_code == "UR"
                    || car.behavior_code == "LU")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "DU"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "DU" || car.behavior_code == "LR")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
            }
        }
    }

    pub fn move_one_step_if_no_collide(&mut self, temp_cars: &mut Vec<Car>) {
        let mut temp_self_car = self.clone();
        temp_cars.retain(|car| temp_self_car.uuid != car.uuid);

        match &*self.current_direction {
            "West" => {
                temp_self_car.car_rect.x -= temp_self_car.current_speed;
                if temp_cars
                    .iter_mut()
                    .all(|car| temp_self_car.car_rect.intersect(car.car_rect).is_none())
                {
                    temp_cars.push(temp_self_car);
                    self.car_rect.x -= self.current_speed;
                }
            }
            "North" => {
                temp_self_car.car_rect.y -= temp_self_car.current_speed;
                if temp_cars
                    .iter_mut()
                    .all(|car| temp_self_car.car_rect.intersect(car.car_rect).is_none())
                {
                    temp_cars.push(temp_self_car);
                    self.car_rect.y -= self.current_speed;
                }
            }
            "South" => self.car_rect.y += self.current_speed,
            "East" => self.car_rect.x += self.current_speed,
            _ => {}
        };
    }

    pub fn update_radar(&mut self, car_index: usize, temp_cars: &Vec<Car>) {
        match &*self.current_direction {
            "West" => {
                // Update radar rectangle
                (self.radar.x, self.radar.y) =
                    (self.car_rect.x - self.radar_size.long_edge, self.car_rect.y);
                (self.radar.w, self.radar.h) =
                    (self.radar_size.long_edge, self.radar_size.short_edge);

                // Reposition the radar when intersection occur
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        self.radar.x = other_car.car_rect.x + other_car.car_rect.w;
                    }
                    // Update radar width
                    self.radar.w = (self.car_rect.x - self.radar.x).abs().min(43.);
                }
            }
            "North" => {
                // Update radar rectangle
                (self.radar.x, self.radar.y) =
                    (self.car_rect.x, self.car_rect.y - self.radar_size.long_edge);
                //Reposition the radar when intersection occur
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && (self.radar.intersect(other_car.car_rect).is_some()
                            || (other_car.behavior_code == "LR"
                                && self.radar.intersect(other_car.radar).is_some()))
                    {
                        self.radar.y = other_car.car_rect.y + other_car.car_rect.h;
                    }
                    // Update radar width
                    self.radar.h = (self.car_rect.y - self.radar.y).abs().min(43.);
                    self.radar.w = 33.;
                }
            }
            "South" => {
                // Update radar rectangle

                (self.radar.x, self.radar.y) =
                    (self.car_rect.x, self.car_rect.y + self.radar_size.long_edge);
                (self.radar.w, self.radar.h) =
                    (self.radar_size.short_edge, self.radar_size.long_edge);
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        //self.radar.h = vec2(self.radar.x, self.radar.y).distance(vec2(other_car.car_rect.x, other_car.car_rect.y)).min(self.radar_size.long_edge)
                        self.radar.h =
                            other_car.car_rect.y - (self.car_rect.y + self.car_size.long_edge)
                    }
                }
            }
            "East" => {
                // Update radar rectangle
                (self.radar.x, self.radar.y) = (self.car_rect.x + self.car_rect.w, self.car_rect.y);
                (self.radar.w, self.radar.h) =
                    (self.radar_size.long_edge, self.radar_size.short_edge);

                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        //self.radar.y = other_car.car_rect.y + other_car.car_rect.h;
                        self.radar.w = other_car.car_rect.x - (self.car_rect.x + self.car_rect.w);
                    }
                    if self.uuid != other_car.uuid
                        && self.radar.intersect(other_car.radar).is_some()
                        && self.car_rect.intersect(other_car.radar).is_none()
                        && other_car.current_direction != "North"
                    {
                        self.radar.w = other_car.car_rect.x - (self.car_rect.x + self.car_rect.w);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn adjust_current_speed(&mut self) {
        if &*self.current_direction == "West" || &*self.current_direction == "East" {
            match self.radar.w {
                radar_width if radar_width <= 3. => {
                    self.current_speed = self.randomized_initial_speed * 0.;
                    if radar_width <= 0.75 && radar_width > 0. {
                        self.close_calls = 1;
                    };
                }
                radar_width if radar_width <= 30. => {
                    self.current_speed = self.randomized_initial_speed * 0.25;
                }
                radar_width if radar_width <= 39. => {
                    self.current_speed = self.randomized_initial_speed * 0.50
                }
                _ => self.current_speed = self.randomized_initial_speed,
            }
        } else if &*self.current_direction == "North" || &*self.current_direction == "South" {
            match self.radar.h {
                //radar_height if radar_height <= 4. => self.current_speed = 0.,
                radar_height if radar_height <= 3. => {
                    self.current_speed = 0.;
                    if radar_height <= 0.75 && radar_height > 0. {
                        self.close_calls = 1;
                    };
                }
                radar_height if radar_height <= 20. => {
                    self.current_speed = self.randomized_initial_speed * 0.25;
                }
                radar_height if radar_height <= 39. => {
                    self.current_speed = self.randomized_initial_speed * 0.50;
                }
                _ => self.current_speed = self.randomized_initial_speed,
            }
        } else {
        }
    }

    pub fn turn_if_can(&mut self, temp_cars: &Vec<Car>) {
        if self.has_turned == false && self.behavior_code == "RU" && self.car_rect.x <= 683. {
            self.waiting_flag = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                683.,
                self.car_rect.y - (self.car_rect.w - self.car_rect.h).abs(),
                self.car_rect.h,
                self.car_rect.w,
            );
            for other_car in temp_cars {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.car_rect).is_some()
                        || temp_rect.intersect(other_car.car_rect).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "North".to_string();
                self.has_turned = true;
            }
        }
        if self.has_turned == false && self.behavior_code == "RD" && self.car_rect.x <= 555. {
            self.waiting_flag = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(555., self.car_rect.y, self.car_rect.h, self.car_rect.w);
            for other_car in temp_cars {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.car_rect).is_some()
                        || temp_rect.intersect(other_car.car_rect).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "South".to_string();
                self.has_turned = true;
            }
        }
        if self.has_turned == false && self.behavior_code == "DR" && self.car_rect.y <= 695. {
            self.waiting_flag = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(self.car_rect.x, 695., self.car_rect.h, self.car_rect.w);
            for other_car in temp_cars {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.car_rect).is_some()
                        || temp_rect.intersect(other_car.car_rect).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "East".to_string();
                self.has_turned = true;
            }
        }
        if self.has_turned == false && self.behavior_code == "DL" && self.car_rect.y <= 574. {
            self.waiting_flag = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                self.car_rect.x - (self.car_rect.h - self.car_rect.w).abs(),
                574.,
                self.car_rect.h,
                self.car_rect.w,
            );
            for other_car in temp_cars {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.car_rect).is_some()
                        || temp_rect.intersect(other_car.car_rect).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "West".to_string();
                self.has_turned = true;
            }
        }
        if self.has_turned == false
            && self.behavior_code == "LD"
            && self.car_rect.x + self.car_size.long_edge >= 510.
        {
            self.waiting_flag = true;
            let temp_rect = Rect::new(
                510. - (self.car_size.long_edge - self.car_size.delta_edge),
                self.car_rect.y,
                self.car_size.short_edge,
                self.car_size.long_edge,
            );
            println!("{:?}", self.car_rect);
            self.car_rect = temp_rect;
            self.waiting_flag = false;
            self.current_direction = "South".to_string();
            self.has_turned = true;
            println!("{:?}", self.car_rect);
        }
        if self.has_turned == false
            && self.behavior_code == "LU"
            && self.car_rect.x + self.car_size.delta_edge >= 603.
        {
            self.waiting_flag = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                603.,
                self.car_rect.y - self.car_size.delta_edge,
                self.car_size.short_edge,
                self.car_size.long_edge,
            );
            for other_car in temp_cars {
                if self.uuid != other_car.uuid && temp_rect.intersect(other_car.car_rect).is_some()
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "North".to_string();
                self.has_turned = true;
            }
        }
        if self.has_turned == false
            && self.behavior_code == "UL"
            && self.car_rect.y + self.car_size.long_edge >= 528.
        {
            self.waiting_flag = true;
            let temp_rect = Rect::new(
                self.car_rect.x - self.car_size.delta_edge,
                528. - (self.car_size.long_edge - self.car_size.delta_edge),
                self.car_size.long_edge,
                self.car_size.short_edge,
            );

            self.car_rect = temp_rect;
            self.waiting_flag = false;
            self.current_direction = "West".to_string();
            self.has_turned = true;
        }
        if self.has_turned == false
            && self.behavior_code == "UR"
            && self.car_rect.y + self.car_size.long_edge >= 650.
        {
            self.waiting_flag = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                self.car_rect.x,
                650. - (self.car_size.long_edge - self.car_size.delta_edge),
                self.car_size.long_edge,
                self.car_size.short_edge,
            );
            for other_car in temp_cars {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.car_rect).is_some()
                        || (temp_rect.intersect(other_car.radar).is_some()
                            && other_car.behavior_code == "DL"))
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "East".to_string();
                self.has_turned = true;
            }
        }
    }

    pub fn draw_all_components(&self, car_texture: &Texture2D, debug: bool) {
        if debug {
            // Draw Radar Rect
            draw_rectangle(
                self.radar.x,
                self.radar.y,
                self.radar.w,
                self.radar.h,
                Color::new(1.0, 0.0, 0.0, 0.1),
            );

            // Draw Car Rect
            draw_rectangle(
                self.car_rect.x,
                self.car_rect.y,
                self.car_rect.w,
                self.car_rect.h,
                Color::new(0.0, 1.0, 0.0, 0.3),
            );
        }

        // Draw Car image top of rect
        match &*self.current_direction {
            "West" => draw_texture_ex(
                car_texture,
                self.car_rect.x + 1.5,
                self.car_rect.y + 1.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(40., 30.)),
                    source: None,
                    rotation: 0.,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            ),
            "North" => {
                let degree: f32 = 90.;
                draw_texture_ex(
                    car_texture,
                    self.car_rect.x - 3.,
                    self.car_rect.y + 7.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40., 30.)),
                        source: None,
                        rotation: degree.to_radians(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            }
            "South" => {
                let degree: f32 = 270.;
                draw_texture_ex(
                    car_texture,
                    self.car_rect.x - 3.,
                    self.car_rect.y + 7.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40., 30.)),
                        source: None,
                        rotation: degree.to_radians(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            }
            "East" => {
                let degree: f32 = 180.;
                draw_texture_ex(
                    car_texture,
                    self.car_rect.x + 2.,
                    self.car_rect.y + 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40., 30.)),
                        source: None,
                        rotation: degree.to_radians(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            }
            _ => {}
        }
    }
}

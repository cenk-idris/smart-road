use macroquad::prelude::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Stats {
    pub total_cars: u32,
    pub best_time: f32,
    pub worst_time: f32,
    pub best_velocity: f32,
    pub worst_velocity: f32,
    pub close_calls: u32,
    pub collisions: u32,
}

impl Stats {
    pub fn draw_ingame(&self) {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 15., 100., 32., RED);
        draw_text(
            format!("Total Cars Arrived: {}", self.total_cars).as_str(),
            15.,
            150.,
            32.,
            RED,
        );
        draw_text(
            format!("Best Time: {} sec", self.best_time).as_str(),
            15.,
            200.,
            32.,
            RED,
        );
        draw_text(
            format!("Worst Time: {} sec", self.worst_time).as_str(),
            15.,
            250.,
            32.,
            RED,
        );
        draw_text(
            format!("Best Velocity: {}", self.best_velocity).as_str(),
            15.,
            300.,
            32.,
            RED,
        );
        draw_text(
            format!("Worst Velocity: {}", self.worst_velocity).as_str(),
            15.,
            350.,
            32.,
            RED,
        );
        draw_text(
            format!("Collision: {}", self.collisions).as_str(),
            915.,
            150.,
            32.,
            RED,
        );
        draw_text(
            format!("Close Calls: {}", self.close_calls).as_str(),
            915.,
            200.,
            32.,
            RED,
        );
    }

    pub fn draw_endgame(&self) {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 100., 100., 32., RED);
        draw_text("Statistics", 500., 250., 46., WHITE);
        draw_text(
            format!("Total Cars Arrived: {}", self.total_cars).as_str(),
            450.,
            300.,
            32.,
            RED,
        );
        draw_text(
            format!("Best Time: {} sec", self.best_time).as_str(),
            450.,
            350.,
            32.,
            RED,
        );
        draw_text(
            format!("Worst Time: {} sec", self.worst_time).as_str(),
            450.,
            400.,
            32.,
            RED,
        );
        draw_text(
            format!("Best Velocity: {}", self.best_velocity).as_str(),
            450.,
            450.,
            32.,
            RED,
        );
        draw_text(
            format!("Worst Velocity: {}", self.worst_velocity).as_str(),
            450.,
            500.,
            32.,
            RED,
        );
        draw_text(
            "At this point there is no return, press Esc to exit as if you have a choice :)",
            150.,
            800.,
            24.,
            WHITE,
        );
        draw_text(
            format!("Collision: {}", self.collisions).as_str(),
            850.,
            300.,
            32.,
            RED,
        );
        draw_text(
            format!("Close Calls: {}", self.close_calls).as_str(),
            850.,
            350.,
            32.,
            RED,
        );
    }
}

use js_sys::Array;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Direction, Point, Segment, Snake};

#[derive(Debug)]
#[wasm_bindgen]
pub struct Game {
    width: i32,
    height: i32,
    snake: Snake,
    food: Point,
    score: i32,
    speed: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32, speed: f64) -> Self {
        let head = Point::new(width / 2, height / 2);
        let snake = Snake::new(head);
        let vectors = &snake.clone().get_body();

        Self {
            width,
            height,
            snake,
            food: Game::gen_food(width, height, vectors),
            score: 0,
            speed,
        }
    }

    pub fn restart(&mut self) {
        let head = Point::new(self.width / 2, self.height / 2);
        let snake = self.snake.reset(head);

        let vectors = &snake.clone().get_body();
        self.snake = snake;
        self.food = Game::gen_food(self.width, self.height, vectors);
        self.score = 0;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn get_snake(&self) -> Array {
        self.snake.clone().into()
    }

    pub fn get_food(&self) -> Point {
        self.food.clone()
    }

    pub fn step(&mut self, direction: Option<Direction>) {
        self.snake.step(direction);
        self.process_food()
    }

    pub fn is_over(&self) -> bool {
        // check if snake is out of bounds
        let Point { x, y } = self.snake.get_head();
        if x < 0.0 || x >= self.width as f64 || y < 0.0 || y >= self.height as f64 {
            return true;
        }
        false
    }

    fn process_food(&mut self) {
        let head_segment = self.snake.get_head_segment();
        if head_segment.is_point_inside(&self.food) {
            self.snake.grow();
            self.score += 1;
            let food = Game::gen_food(self.width, self.height, &self.snake.get_body().clone());
            self.food = food;
            self.speed_up();
        }
    }

    fn speed_up(&mut self) {
        match self.score {
            x if x % 5 == 0 => self.speed += 0.1,
            _ => (),
        }
    }

    fn gen_food(width: i32, height: i32, vectors: &Vec<Point>) -> Point {
        let segments = Segment::from_vectors(vectors);
        let mut free_points = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let p = Point::new(x, y);
                if segments.iter().all(|seg| !seg.is_point_inside(&p)) {
                    free_points.push(p)
                }
            }
        }
        let idx = rand::thread_rng().gen_range(0..free_points.len());
        free_points.get(idx).unwrap().clone()
    }
}

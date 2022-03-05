use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

const STEP_DISTANCE: f64 = 1.0;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
        }
    }

    fn subtract(&self, other: &Point) -> Point {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn add(&self, other: &Point) -> Point {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn lenght(&self) -> f64 {
        let val = (self.x).hypot(self.y);
        val
    }

    fn scale(&self, number: f64) -> Point {
        Self {
            x: self.x * number,
            y: self.y * number,
        }
    }

    fn normalize(&self) -> Point {
        Self {
            x: self.x / self.lenght(),
            y: self.y / self.lenght(),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

#[wasm_bindgen]
impl Snake {
    pub fn new(head: Point) -> Self {
        let tail = head.subtract(&Point::new(3, 0));

        Self {
            body: vec![tail, head],
            direction: Direction::default(),
        }
    }

    /**
     *  seg: [(1, 1), (4, 1)]
     *  
     */
    pub fn step(&mut self, direction: Option<Direction>) {
        let mut body = Vec::new();
        // 处理tail
        let tail = self.body.remove(0);
        let point = self.body.get(0).unwrap();
        let segment = Segment::new(&tail, point);

        let next_tail: Point;
        if segment.length() as i32 > 1 {
            let vector = segment.vector().normalize().scale(STEP_DISTANCE);
            next_tail = tail.add(&vector);

            body.push(next_tail);
        }
        body.append(&mut self.body);

        // 处理head
        // println!("aaaa {:?}", self.body);
        let head = body.pop().unwrap();
        let mut next_head: Point;
        next_head = head.add(&self.direction.into_vector());
        if let Some(dir) = direction {
            if self.direction.is_turn_around(&dir) {
                let vector = dir.into_vector();
                next_head = head.add(&vector);
                body.push(head);
                self.direction = dir;
            }
        }
        body.push(next_head);
        self.body = body;
    }
}

impl Snake {
    pub fn get_body(&self) -> Vec<Point> {
        self.body.clone()
    }

    pub fn get_head(&self) -> Point {
        let size = self.body.len();
        self.body[size - 1].clone()
    }

    pub fn get_head_segment(&self) -> Segment {
        let size = self.body.len();
        let start = &self.body[size - 2];
        let end = &self.body[size - 1];
        Segment::new(start, end)
    }

    pub fn grow(&mut self) {
        let tail = &self.body[0];
        let next = &self.body[1];
        let tail_segment = Segment::new(next, tail);
        let new_tail = tail.add(&tail_segment.vector().normalize().scale(STEP_DISTANCE));
        self.body[0] = new_tail;
    }

    pub fn reset(&mut self, head: Point) -> Self {
        let tail = head.subtract(&Point::new(3, 0));

        Self {
            body: vec![tail, head],
            direction: Direction::default(),
        }
    }
}

impl From<Snake> for Array {
    fn from(s: Snake) -> Self {
        s.body.clone().into_iter().map(JsValue::from).collect()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[wasm_bindgen]
impl Direction {
    fn into_vector(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    fn is_turn_around(&self, other: &Direction) -> bool {
        if self == other {
            println!("same");
            return false;
        }
        if self.into_vector().add(&other.into_vector()) == Point::new(0, 0) {
            println!("opposite");
            return false;
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct Segment<'a> {
    start: &'a Point,
    end: &'a Point,
}

impl<'a> Segment<'a> {
    fn new(start: &'a Point, end: &'a Point) -> Self {
        Self { start, end }
    }

    fn vector(&self) -> Point {
        self.end.subtract(&self.start)
    }

    pub fn length(&self) -> f64 {
        // segment目前只有横竖 两种， length就是线段长度，不是三角形第三边
        self.vector().lenght()
    }

    pub fn is_point_inside(&self, point: &Point) -> bool {
        let first = Segment::new(self.start, point);
        let second = Segment::new(point, self.end);
        println!("first {:?}: {:?}", &first, &first.length());
        println!("second {:?}: {:?}", &second, &second.length());
        println!("self {:?}: {:?}", &self, &self.length());
        println!("{}", first.length() + second.length());
        f64_equal(self.length(), first.length() + second.length())
        // self.length() == first.length() + second.length()
    }

    pub fn from_vectors(vectors: &Vec<Point>) -> Vec<Segment> {
        let pairs = vectors[..vectors.len() - 1].iter().zip(&vectors[1..]);
        pairs
            .map(|(s, e)| Segment::new(s, e))
            .collect::<Vec<Segment>>()
    }
}

fn f64_equal(x: f64, y: f64) -> bool {
    (x - y).abs() < f64::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Snake;

    #[test]
    fn step_works() {
        let mut snake = Snake::new(Point::new(5, 5));
        snake.body.insert(0, Point::new(2, 3));

        snake.step(Some(Direction::Up));
        let espect = vec![
            Point { x: 2.0, y: 4.0 },
            Point { x: 2.0, y: 5.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 5.0, y: 4.0 },
        ];
        println!("{:?}", snake.body);
        assert_eq!(espect, snake.body)
    }

    #[test]
    fn is_point_inside_test() {
        let start = Point::new(0, 0);
        let end = Point::new(30, 30);
        let seg = Segment::new(&start, &end);
        let p1 = Point::new(2, 2);
        let p2 = Point::new(1, 0);

        let flag1 = seg.is_point_inside(&p1);
        let flag2 = seg.is_point_inside(&p2);

        assert_eq!(true, flag1);
        assert_eq!(false, flag2)
    }
}

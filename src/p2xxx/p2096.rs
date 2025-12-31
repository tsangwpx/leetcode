// Problem 2069

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Robot {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    face: Direction,
    steps: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            x: 0,
            y: 0,
            face: Direction::East,
            steps: 0,
        }
    }

    fn step(&mut self, num: i32) {
        self.steps += num;

        if self.steps >= 10i32.pow(8) {
            self._update();
        }
    }

    fn _update(&mut self) {
        if self.steps == 0 {
            // do nothing
            return;
        }

        let cycle = (self.width + self.height) * 2 - 4;
        self.steps = self.steps % cycle;
        if self.steps == 0 {
            // go back to the original position.
            // but may be facing different direction
            // so call an inner update with (cycle - 1) steps
            self.steps = cycle - 1;
            self._update();
            self.steps = 1;
        }

        let mut rem = self.steps;
        while rem > 0 {
            let (dx, dy) = match self.face {
                Direction::North => (0, rem.min(self.height - self.y - 1)),
                Direction::East => (rem.min(self.width - self.x - 1), 0),
                Direction::South => (0, -self.y.min(rem)),
                Direction::West => (-self.x.min(rem), 0),
            };

            if dx == 0 && dy == 0 {
                self.face = match self.face {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                };
            } else {
                self.x += dx;
                self.y += dy;

                rem = rem - dx.abs() - dy.abs()
            }
        }

        self.steps = 0;
    }

    fn get_pos(&mut self) -> Vec<i32> {
        self._update();
        vec![self.x, self.y]
    }

    fn get_dir(&mut self) -> String {
        self._update();
        match self.face {
            Direction::North => "North".to_owned(),
            Direction::East => "East".to_owned(),
            Direction::South => "South".to_owned(),
            Direction::West => "West".to_owned(),
        }
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
fn _f() {}

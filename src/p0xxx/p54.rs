#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Walker {
    direction: Direction,
    pos_x: usize,
    pos_y: usize,
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

impl Walker {
    fn new(m: usize, n: usize) -> Self {
        assert!(m >= 1 && n >= 1);

        Self {
            direction: Direction::East,
            pos_x: 0,
            pos_y: 0,
            top: 0,
            right: n - 1,
            bottom: m - 1,
            left: 0,
        }
    }

    fn _dead_end(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => self.pos_y == self.top,
            Direction::East => self.pos_x == self.right,
            Direction::South => self.pos_y == self.bottom,
            Direction::West => self.pos_x == self.left,
        }
    }

    fn _what_next(direction: Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn step(&mut self) -> Result<(), ()> {
        let mut direction = self.direction;
        if self._dead_end(direction) {
            direction = Self::_what_next(direction);

            if self._dead_end(direction) {
                return Err(());
            }

            self.direction = direction;

            match direction {
                Direction::North => self.bottom -= 1,
                Direction::East => self.left += 1,
                Direction::South => self.top += 1,
                Direction::West => self.right -= 1,
            }
        }

        match direction {
            Direction::North => {
                self.pos_y -= 1;
            }
            Direction::East => {
                self.pos_x += 1;
            }
            Direction::South => {
                self.pos_y += 1;
            }
            Direction::West => {
                self.pos_x -= 1;
            }
        }

        Ok(())
    }
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut order = Vec::with_capacity(m * n);

        let mut i = 0;
        let mut j = 0;
        let mut top = 0;
        let mut right = n - 1;
        let mut bottom = m - 1;
        let mut left = 0;

        order.push(unsafe { *matrix.get_unchecked(j).get_unchecked(i) });

        while top <= bottom && left <= right {
            // Eastward
            // println!("east {} {}: {} {} {} {}", i, j, top, right, bottom, left);
            while i + 1 <= right {
                i += 1;
                order.push(unsafe { *matrix.get_unchecked(j).get_unchecked(i) });
            }
            if top == bottom {
                break;
            }
            top += 1;

            // Southward
            // println!("south {} {}: {} {} {} {}", i, j, top, right, bottom, left);
            while j + 1 <= bottom {
                j += 1;
                order.push(unsafe { *matrix.get_unchecked(j).get_unchecked(i) });
            }
            if left == right {
                break;
            }
            right -= 1;

            // Westward
            // println!("west {} {}: {} {} {} {}", i, j, top, right, bottom, left);
            while i > left {
                i -= 1;
                order.push(unsafe { *matrix.get_unchecked(j).get_unchecked(i) });
            }
            if top == bottom {
                break;
            }
            bottom -= 1;

            // Northward
            // println!("north {} {}: {} {} {} {}", i, j, top, right, bottom, left);
            while j > top {
                j -= 1;
                order.push(unsafe { *matrix.get_unchecked(j).get_unchecked(i) });
            }
            if left == right {
                break;
            }
            left += 1;
        }

        order
    }

    pub fn spiral_order2(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut walker = Walker::new(m, n);
        let mut order = Vec::with_capacity(m * n);

        loop {
            order.push(matrix[walker.pos_y][walker.pos_x]);

            match walker.step() {
                Ok(_) => continue,
                Err(_) => return order,
            }
        }
    }
}

fn main() {}

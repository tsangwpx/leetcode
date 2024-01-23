// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 48
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let shells = n / 2;

        for layer in 0..shells {
            for k in layer..(n - layer - 1) {
                // clockwise
                let (r1, c1) = (layer, k);
                let (r2, c2) = (c1, n - r1 - 1);
                let (r3, c3) = (c2, n - r2 - 1);
                let (r4, c4) = (c3, n - r3 - 1);

                println!("{} {}: {:?}", layer, k, (r1, c1, r2, c2, r3, c3, r4, c4));

                // anti-clockwise
                let memo = matrix[r1][c1];
                matrix[r1][c1] = matrix[r4][c4];
                matrix[r4][c4] = matrix[r3][c3];
                matrix[r3][c3] = matrix[r2][c2];
                matrix[r2][c2] = memo;
            }
        }
    }
}

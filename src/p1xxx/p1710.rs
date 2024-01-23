impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|vec| {
            -vec[1]
        });

        let mut units = 0;
        let mut space = truck_size;

        for box_type in box_types {
            assert_eq!(box_type.len(), 2);
            let taken = space.min(box_type[0]);
            space -= taken;
            units += taken * box_type[1];

            if space == 0 {
                break;
            }
        }

        units
    }
}

struct Solution {}

fn main() {
    println!("Hello World");
}
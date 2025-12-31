// Problem 383
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut counter = [0i32; 256];
        ransom_note.bytes().for_each(|s| counter[s as usize] += 1);

        magazine.bytes().for_each(|s| counter[s as usize] -= 1);

        counter.iter().all(|&s| s <= 0)
    }
}

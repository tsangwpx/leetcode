// Problem 2285
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        // Count the linkages of cities
        let mut links: Vec<u32> = vec![0; n as usize];
        for (a, b) in roads.iter().map(|x| (x[0] as usize, x[1] as usize)) {
            links[a] += 1;
            links[b] += 1;
        }

        // map rank/index to city/value; larger index higher ranks more more important city
        let mut ranks: Vec<u32> = (0..n as u32).collect();
        ranks.sort_by_key(|&city| links[city as usize]);

        let mut total: i64 = 0;

        for (importance, city) in ranks.iter().enumerate() {
            let importance = (importance + 1) as u32;
            total += (links[*city as usize] * importance) as i64;
        }

        total
    }
}

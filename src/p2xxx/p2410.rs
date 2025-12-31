// Problem 2410
impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut matches = 0;

        while i < players.len() && j < trainers.len() {
            let ability = players[i];
            let capacity = trainers[j];

            if ability <= capacity {
                i += 1;
                j += 1;
                matches += 1;
            } else {
                j += 1;
            }
        }

        matches
    }
}

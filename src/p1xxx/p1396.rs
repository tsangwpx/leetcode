// Problem 1396
use std::collections::HashMap;

#[derive(Clone, Copy, Default, Debug)]
struct AverageTime {
    total: i64,
    count: i32,
}

struct UndergroundSystem {
    customers: HashMap<i32, (String, i32)>,
    average_map: HashMap<(String, String), AverageTime>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            customers: HashMap::new(),
            average_map: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        match self.customers.entry(id) {
            std::collections::hash_map::Entry::Occupied(_entry) => {
                panic!("Customer has checked in!");
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert((station_name, t));
            }
        }
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, start_time) = self
            .customers
            .remove(&id)
            .expect("Customer has not checked in");

        let end_station = station_name;
        let duration = t - start_time;

        assert!(duration >= 0, "inconsistent time");

        let avg_time = self
            .average_map
            .entry((start_station, end_station))
            .or_default();

        avg_time.total += i64::from(duration);
        avg_time.count += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let avg_time = self
            .average_map
            .get(&(start_station, end_station))
            .expect("No record");

        avg_time.total as f64 / avg_time.count as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
fn _comment() {}

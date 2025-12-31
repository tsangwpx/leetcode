// Problem 2092
impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // union-find + reset between time iteration

        meetings.sort_unstable_by_key(|s| s[2]);
        let meetings = meetings;

        let mut groups = (0..n).collect::<Vec<Item>>();

        type Item = i32;
        let n = n as usize;

        #[inline]
        fn find(groups: &mut Vec<Item>, index: i32) -> i32 {
            let mut parent = groups[index as usize];

            if parent != groups[parent as usize] {
                parent = find(groups, parent);
                groups[index as usize] = parent;
            }

            parent
        }

        #[inline]
        fn union(groups: &mut Vec<Item>, i: i32, j: i32) -> i32 {
            let i = find(groups, i);
            let j = find(groups, j);
            let mn = i.min(j);
            let mx = i.max(j);
            groups[mx as usize] = mn;
            mn
        }

        union(&mut groups, 0, first_person);

        let mut last_time = -1;
        let mut last_people = Vec::with_capacity(groups.len());

        for row in meetings.iter() {
            let &[x, y, time] = row.as_slice() else {
                panic!("bad format")
            };

            let time_changed = time != last_time;

            if time_changed {
                for person in last_people.drain(..) {
                    if find(&mut groups, person) != 0 {
                        // reset their group if they dont know the secret!
                        groups[person as usize] = person;
                    }
                }
            }

            last_time = time;

            let res = union(&mut groups, x, y);

            if res != 0 {
                // skip adding them if they know the secret. Optimization
                last_people.push(x);
                last_people.push(y);
            }
        }

        (0..n as i32)
            .filter_map(|i| {
                let root = find(&mut groups, i);

                if root == 0 { Some(i) } else { None }
            })
            .collect::<Vec<_>>()
    }
}

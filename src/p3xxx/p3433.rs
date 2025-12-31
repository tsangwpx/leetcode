// Problem 3433
impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        use std::convert::TryInto;

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum Message {
            Some(String),
            All,
            Here,
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum Event {
            Online(i32),
            Offline(i32),
            Message(Message),
        }

        let mut all_count = 0;
        let mut seq = Vec::with_capacity(events.len());

        for event in events.into_iter() {
            assert_eq!(event.len(), 3);

            let Ok([type_, time, spec]) = <Vec<_> as TryInto<[String; 3]>>::try_into(event) else {
                unreachable!()
            };

            let time = time.parse::<i32>().expect("time");

            if type_ == "MESSAGE" {
                let msg = match spec.as_str() {
                    "ALL" => {
                        all_count += 1;
                        continue;
                    }
                    "HERE" => Message::Here,
                    _ => Message::Some(spec),
                };

                seq.push((time, Event::Message(msg)));
            } else if type_ == "OFFLINE" {
                let user_id = spec.parse::<i32>().expect("user_id");

                seq.push((time + 60, Event::Online(user_id)));
                seq.push((time, Event::Offline(user_id)));
            } else {
                unreachable!()
            }
        }

        seq.sort_unstable();

        let user_count = number_of_users as usize;

        let mut mentions = vec![all_count; user_count];
        let mut online = vec![true; user_count];

        for (_, event) in seq.iter() {
            match event {
                &Event::Online(user_id) => online[user_id as usize] = true,
                &Event::Offline(user_id) => online[user_id as usize] = false,
                Event::Message(message) => match message {
                    Message::Some(spec) => {
                        for part in spec.split(' ') {
                            let user_id = part[2..].parse::<i32>().expect("user_id");
                            mentions[user_id as usize] += 1;
                        }
                    }
                    Message::All => {
                        unreachable!();
                    }
                    Message::Here => {
                        for (idx, here) in online.iter().copied().enumerate() {
                            if here {
                                mentions[idx] += 1;
                            }
                        }
                    }
                },
            }
        }

        mentions
    }
}

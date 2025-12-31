// Problem 355
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Tweet {
    time: i32,
    tweet_id: i32,
    user_id: i32,
}

#[derive(Debug)]
struct User {
    tweets: VecDeque<Tweet>,
    feed_time: i32,
    feed_tweets: BinaryHeap<Reverse<Tweet>>,
    followings: HashSet<i32>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            tweets: Default::default(),
            feed_time: -1,
            feed_tweets: Default::default(),
            followings: Default::default(),
        }
    }
}

#[derive(Debug)]
struct Twitter {
    limit: usize,
    time: i32,
    users: HashMap<i32, RefCell<User>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    const TWEET_LIMIT: usize = 10;

    fn new() -> Self {
        Self {
            limit: Self::TWEET_LIMIT,
            time: 1,
            users: Default::default(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let now = self.time;
        self.time += 1;

        let user = self.users.entry(user_id).or_default();
        let user = &mut *RefCell::borrow_mut(user);

        user.tweets.truncate(self.limit - 1);

        let tweet = Tweet {
            time: now,
            tweet_id,
            user_id,
        };

        user.tweets.push_front(tweet);

        // empty the feed
        user.feed_time = -1;
        user.feed_tweets.clear();
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        self.users.entry(user_id).or_default();

        let user = self.users.get(&user_id).unwrap();
        let user = &mut *RefCell::borrow_mut(&user);

        if user.feed_time < 0 {
            user.feed_tweets.clear();
            user.feed_tweets
                .extend(user.tweets.iter().copied().map(|s| Reverse(s)));
        }

        let mut latest = user.feed_time;

        for following in user.followings.iter().copied() {
            let Some(other) = self.users.get(&following) else {
                continue;
            };
            let other = RefCell::borrow(other);

            for tweet in other.tweets.iter().copied() {
                if tweet.time <= user.feed_time {
                    break;
                }

                latest = latest.max(tweet.time);
                if user.feed_tweets.len() > self.limit {
                    user.feed_tweets.pop();
                }

                user.feed_tweets.push(Reverse(tweet));
            }
        }

        if user.feed_tweets.len() > self.limit {
            user.feed_tweets.pop();
        }

        user.feed_time = latest;

        let tweets = user
            .feed_tweets
            .clone()
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(s)| s.tweet_id)
            .collect::<Vec<_>>();

        tweets
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }

        self.users.entry(followee_id).or_default();

        let user = self.users.entry(follower_id).or_default();
        let user = &mut *RefCell::borrow_mut(user);

        if !user.followings.insert(followee_id) {
            // already added
            return;
        }

        // empty the feed
        user.feed_time = -1;
        user.feed_tweets.clear();
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            // cant un-follow oneself
            return;
        }

        self.users.entry(followee_id).or_default();

        let user = self.users.entry(follower_id).or_default();
        let user = &mut *RefCell::borrow_mut(user);

        if !user.followings.remove(&followee_id) {
            // non-existential
            return;
        }

        // empty the feed
        user.feed_time = -1;
        user.feed_tweets.clear();
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
fn __() {}

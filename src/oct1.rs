struct RecentCounter {
    pings: Vec<i32>,
}

const WINDOW: i32 = 3000;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self { pings: Vec::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push(t);
        self.pings.retain(|&x| x >= t - WINDOW);
        self.pings.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod test {
    use super::RecentCounter;

    #[test]
    fn example1() {
        let mut counter = RecentCounter::new();
        let mut x = counter.ping(1);
        assert_eq!(x, 1);
        x = counter.ping(100);
        assert_eq!(x, 2);
        x = counter.ping(3001);
        assert_eq!(x, 3);
        x = counter.ping(3002);
        assert_eq!(x, 3);
    }
}

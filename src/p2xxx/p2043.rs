// Problem 2043
struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let idx1 = account1 as usize - 1;
        let idx2 = account2 as usize - 1;

        if idx1 >= self.balance.len() || idx2 >= self.balance.len() {
            return false;
        }

        if money > self.balance[idx1] {
            return false;
        }

        self.balance[idx1] -= money;
        self.balance[idx2] += money;

        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let idx = account as usize - 1;
        if idx >= self.balance.len() || money < 0 {
            return false;
        }

        self.balance[idx] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let idx = account as usize - 1;
        if idx >= self.balance.len() || self.balance[idx] < money {
            return false;
        }

        self.balance[idx] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
fn f() {}

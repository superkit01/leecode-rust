use std::cell::RefCell;

pub struct Bank {
    balance: RefCell<Vec<i64>>,
}

impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Bank {
            balance: RefCell::new(balance),
        }
    }

    pub fn transfer(&self, account1: i32, account2: i32, money: i64) -> bool {
        let mut mut_balance = self.balance.borrow_mut();
        if account1 > mut_balance.len() as i32 || account2 > mut_balance.len() as i32 {
            return false;
        }
        if mut_balance[(account1 - 1) as usize] < money {
            return false;
        }
        mut_balance[(account1 - 1) as usize] -= money;
        mut_balance[(account2 - 1) as usize] += money;
        return true;
    }

    pub fn deposit(&self, account: i32, money: i64) -> bool {
        let mut  mut_balance = self.balance.borrow_mut();
        if account > mut_balance.len() as i32 {
            return false;
        }

        mut_balance[(account - 1) as usize] += money;
        return true;
    }

    pub fn withdraw(&self, account: i32, money: i64) -> bool {
        let mut mut_balance = self.balance.borrow_mut();
        if account > mut_balance.len() as i32 {
            return false;
        }
        if mut_balance[(account - 1) as usize] < money {
            return false;
        }

        mut_balance[(account - 1) as usize] -= money;
        return true;
    }
}


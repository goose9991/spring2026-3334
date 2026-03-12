#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!((account.balance() - 150.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit_negative() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert!((account.balance() - 50.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_too_much() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(150.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_negative() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_float_deposit() {
        let mut account = BankAccount::new(100.25);
        account.deposit(10.75);

        let expected = 111.0;
        assert!((account.balance() - expected).abs() < EPSILON);
    }

    #[test]
    fn test_float_withdraw() {
        let mut account = BankAccount::new(200.50);
        account.withdraw(50.25);

        let expected = 150.25;
        assert!((account.balance() - expected).abs() < EPSILON);
    }

    #[test]
    fn test_multiple_float_operations() {
        let mut account = BankAccount::new(100.10);

        account.deposit(0.20);
        account.withdraw(0.10);

        let expected = 100.20;
        assert!((account.balance() - expected).abs() < EPSILON);
    }
}
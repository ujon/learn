#[derive(Debug, Clone)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn print_bank(bank: Bank) {
    println!("(2) {:#?}", bank);
}
fn print_account(account: Account) {
    println!("(3) {:#?}", account);
}
fn print_account_borrowing(account: &Account) {
    println!("(2) {:#?}", account);
}

fn print_accounts(accounts: &Vec<Account>) {
    println!("(1) {:#?}", accounts);
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}


fn main() {
    let bank = Bank::new();

    let account = Account::new(1, String::from("Lux"));
    let account1 = Account::new(1, String::from("Nox"));

    let mut other_bank = bank;

    print_accounts(&other_bank.accounts);
    add_account(&mut other_bank, account1);
    print_accounts(&other_bank.accounts);
    add_account(&mut other_bank, account.clone());
    print_accounts(&other_bank.accounts);

    // error: 'Bank' value owner changed from bank to other_bank (ownership)
    // print_bank(bank);
    print_bank(other_bank);

    print_account_borrowing(&account);
    print_account_borrowing(&account);
    print_account(account);
    // error: Value used after being moved
    // print_account(account);

    let num = 5;
    let other_num = num;

    // numbers, bool, arrays, char, tuples, references is available to copy
    println!("(4) {} {} ", num, other_num);

    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Void"));

    account.deposit(200);
    account.withdraw(100);

    bank.add_account(account);
    println!("(5) {:#?}", &bank);

}

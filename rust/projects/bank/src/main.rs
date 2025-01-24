#[derive(Debug)]
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
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_bank(bank: Bank) {
    println!("(2) {:#?}", bank);
}
fn print_account(account: Account) {
    println!("(2) {:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Lux"));

    print_bank(bank);
    print_account(account);
}

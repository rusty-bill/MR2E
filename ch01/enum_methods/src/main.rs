// enum_methods
enum PaymentMode {
    Debit,
    Credit,
    Paypal,
    Bitcoin
}
// bunch of dummy payment handlers
fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of {}", amt);
}

fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of {}", amt);
}

fn pay_by_paypal(amt: u64) {
    println!("Processing PayPal payment of {}", amt);
}

fn pay_by_bitcoin(amt: u64) {
    println!("Processing Bitcoin payment of {}", amt);
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => pay_by_paypal(amount),
            PaymentMode::Bitcoin => pay_by_bitcoin(amount),
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Credit
}

fn main() {
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(100);
}

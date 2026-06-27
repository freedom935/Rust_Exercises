trait Payment {
    fn pay(&self, amount: f64);
}

struct CreditCard {
    owner: String,
}

struct Bitcoin {
    wallet: String,
}

impl Payment for CreditCard {
    fn pay(&self, amount: f64) {
        println!("{} paid ${} with Credit Card", self.owner, amount);
    }
}

impl Payment for Bitcoin {
    fn pay(&self, amount: f64) {
        println!("{} paid ${} with Bitcoin", self.wallet, amount);
    }
}

fn checkout(payment: &impl Payment, amount: f64) {
    payment.pay(amount);
}

fn main() {
    let card = CreditCard {
        owner: "Alice".to_string(),
    };

    let btc = Bitcoin {
        wallet: "bc1qp6ejw8ptj9l9pkscmlf8fhhkrrjeawgpyjvtq8".to_string(),
    };

    checkout(&card, 120.0);
    checkout(&btc, 85.5);
}

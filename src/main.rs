fn main() {
    let netflix = Subscription {
        name: String::from("Netflix"),
        payment_method: UPI::new("a@b", "x@y"),
    };
    println!(
        "Payment Processed for {} with reference {:}",
        netflix.name,
        netflix.process_payment()
    );

    let amazon_prime = Subscription {
        name: String::from("Amazon Prime"),
        payment_method: CreditCard::new(12345678890003, 10, 23),
    };
    println!(
        "Payment Processed for {} with reference {:}",
        amazon_prime.name,
        amazon_prime.process_payment()
    );
}

trait PaymentMethod {
    fn charge(&self) -> String;
}

struct UPI {
    from: String,
    to: String,
}

impl UPI {
    pub fn new(from: &str, to: &str) -> UPI {
        UPI {
            from: String::from(from),
            to: String::from(to),
        }
    }
}

impl PaymentMethod for UPI {
    fn charge(&self) -> std::string::String {
        String::from("UPI")
    }
}

struct CreditCard {
    number: u64,
    expiry: (u8, u8),
}
impl CreditCard {
    pub fn new(number: u64, expiry_month: u8, expiry_year: u8) -> CreditCard {
        CreditCard {
            number,
            expiry: (expiry_month, expiry_year),
        }
    }
}

impl PaymentMethod for CreditCard {
    fn charge(&self) -> std::string::String {
        String::from("Credit Card")
    }
}

trait Purchase<T>
where
    T: PaymentMethod,
{
    fn payment_method(&self) -> &T;
    fn process_payment(&self) -> String {
        let payment_method: &T = self.payment_method();
        payment_method.charge()
    }
}

struct Subscription<T>
where
    T: PaymentMethod,
{
    name: String,
    payment_method: T,
}

impl<T> Purchase<T> for Subscription<T>
where
    T: PaymentMethod,
{
    fn payment_method(&self) -> &T {
        &self.payment_method
    }
}

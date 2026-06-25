
#[derive(Debug)]
enum PaymentMode {
    CashOnDelivery,
    Upi(String),
    CreditCard,
}

#[derive(Debug)]
enum OrderState {
    Pending,
    Paid(PaymentMode),
    Shipped(String),
    Cancelled(String),
}

#[derive(Debug)]
struct Order {
    order_id: u32,
    amount: f64,
    state: OrderState,
}

impl Order {
    fn new(order_id: u32, amount: f64) -> Self {
        Self {
            order_id,
            amount,
            state: OrderState::Pending,
        }
    }

    fn process_payment(&mut self, method: PaymentMode) -> Result<(), &'static str> {
        match self.state {
            OrderState::Pending => {
                self.state = OrderState::Paid(method);
                Ok(())
            }
            OrderState::Paid(_) => Err("Payment cancelled: Order is already paid!"),
            OrderState::Shipped(_) => Err("Payment cancelled: Order is already shipped!"),
            OrderState::Cancelled(_) => Err("Payment cancelled: Order is cancelled!"),
        }
    }

    fn ship_order(&mut self, tracking_id: String) -> Result<(), &'static str> {
        match self.state {
            OrderState::Paid(_) => {
                self.state = OrderState::Shipped(tracking_id);
                Ok(())
            }
            _ => Err("Request denied: Order must be paid before shipping!"),
        }
    }

    fn get_tracking_id(&self) -> Option<&String> {
        if let OrderState::Shipped(tracking) = &self.state {
            return Some(tracking);
        }
        None
    }
}

fn main() {
    let mut order1 = Order::new(20260, 560.00);
    
    let method1 = PaymentMode::Upi(String::from("abc@upi"));
    
    if let Err(e) = order1.process_payment(method1) {
        println!("Error: {}", e);
    } else {
        println!("Order payment successful!");
    }

    if let Err(e) = order1.ship_order(String::from("XYZ12345")) {
        println!("Error: {}", e);
    } else {
        println!("Order shipped successfully!");
    }

    if let Some(t_id) = order1.get_tracking_id() {
        println!("Tracking ID is: {}", t_id);
    }
}
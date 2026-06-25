#[derive(Debug)]
struct ServerNode {
    ip: String,
    port_number: u16,
    active_connection: u32,
    max_connection: u32,
}

impl ServerNode {
    fn new(ip: String, port_number: u16, max_connection: u32) -> Self {
        Self {
            ip,
            port_number,
            active_connection: 0,
            max_connection,
        }
    }

    fn is_available(&self) -> bool {
        self.max_connection > self.active_connection
    }

    fn route_request(&mut self) -> bool {
        if self.is_available() {
            self.active_connection += 1;
            return true;
        }
        false
    }

    fn resolve_request(&mut self) -> bool {
        if self.active_connection > 0 {
            self.active_connection -= 1;
            return true;
        }
        false
    }

    fn has_less_load(&self, node2: &Self) -> bool {
        self.active_connection < node2.active_connection
    }
}

fn main() {
    let mut node1 = ServerNode::new(String::from("127.0.0.1"), 8080, 10);
    let node2 = ServerNode::new(String::from("192.168.1.5"), 443, 10);

    println!("{:#?}", node1);
    println!("server is available ?: {}", node1.is_available());
    println!("is request routed ?: {}", node1.route_request());
    println!("active connection: {}", node1.active_connection);
    println!("is request resolved ?: {}", node1.resolve_request());
    println!("active connection: {}", node1.active_connection);
    println!(
        "is node1 has less load than node2 ?: {}",
        node1.has_less_load(&node2)
    );
}
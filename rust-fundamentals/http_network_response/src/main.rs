#[derive(Debug,Clone)]
struct User {
    username: String,
    id: String,
}
impl User {
    fn new(username: String, id: String) -> Self {
        Self { username, id }
    }
}
#[derive(Debug)]
enum HTTP {
    Success(User),
    NotFound,
    ServerError(String),
}
fn server(user_id: &str, data: &[User; 3]) -> HTTP {
    for i in data {
        if i.id == user_id {
            return HTTP::Success(i.clone());
        }
    }
    return HTTP::NotFound;
}

fn main() {
    let user: [User; 3] = [
        (User::new(String::from("zubair"), String::from("123"))),
        (User::new(String::from("sarik"), String::from("456"))),
        (User::new(String::from("salman"), String::from("789"))),
    ];
    println!("{:?}", user[0]);
    let result = server("123", &user);
    println!("{:?}", result);
}
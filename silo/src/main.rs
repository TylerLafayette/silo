use silo_core::service::Service;

fn main() {
    let service = Service::new();
    match service.run() {
        Ok(_) => println!("hello world"),
        Err(e) => println!("{}", e),
    };
}

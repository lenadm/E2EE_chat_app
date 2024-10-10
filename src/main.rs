mod listener;

fn main() {
    let addr = String::from("127.0.0.1:5224");
    let listener = listener::Listener::new(&addr);

    listener.start_listening();
}

#[derive(Debug)]
enum Message {
    Resize: "resize",
    Move: "move",
    Echo: "echo",
    ChangeColor: "change color",
    Quit: "quit"
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}

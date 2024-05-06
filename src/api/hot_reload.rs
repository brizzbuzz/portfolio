#[get("/hot-reload")]
pub fn index(ws: ws::WebSocket) -> ws::Stream!['static] {
    println!("WebSocket connection initiated.");
    ws::Stream! { ws =>
        for await message in ws {
            println!("Received message: {:?}", message);
            yield message?;
        }
    }
}

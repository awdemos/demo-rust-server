use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind to localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server running at http://127.0.0.1:3000");
    println!("Press Ctrl+C to stop the server");

    // Keep accepting connections
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("New connection from: {}", addr);
                
                // Spawn a new task for each connection
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(socket).await {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    
    // Read the incoming request
    let n = socket.read(&mut buffer).await?;
    println!("Received request of {} bytes", n);
    
    if n == 0 {
        return Ok(());
    }

    // Create HTTP response
    let response = "HTTP/1.1 200 OK\r\n\
                   Content-Type: text/html\r\n\
                   Connection: keep-alive\r\n\
                   \r\n\
                   <html>\
                   <head><title>Rust Server</title></head>\
                   <body>\
                   <h1>Hello from Rust!</h1>\
                   <p>Your web server is working!</p>\
                   </body>\
                   </html>";

    // Write the response
    socket.write_all(response.as_bytes()).await?;
    socket.flush().await?;
    println!("Response sent successfully");

    Ok(())
}
use async_std::io::{WriteExt, ReadExt};
use async_std::net::TcpStream;
use async_std::sync::{Mutex, Arc};
use async_std::{task, io, net::TcpListener};
use async_std::stream::StreamExt;

async fn connection_handler(mut stream: TcpStream, connections: Arc<Mutex<Vec<TcpStream>>> ) -> io::Result<()>{
    let addr = stream.peer_addr()?;
    println!("New Connection: {}", addr);

    // prompt for a name 
    let name = format!("{}: ", addr);

    loop {
        // create a new buffer each time and add the user id/name
        let mut buffer = [0u8; 1024];
        let len = stream.read(&mut buffer).await?;

        if len > 0 {
            println!("{}", String::from_utf8_lossy(&buffer[..len]));

            let connections_guard = connections.lock().await;
            for mut client in connections_guard.iter() {
                // don't send to self
                if client.peer_addr()? != stream.peer_addr()? {
                    client.write(name.as_bytes()).await?;
                    client.write(&buffer).await?;
                }
            }
        } else {
            println!("Disconnected: {}, connections: {}", stream.peer_addr()?, 0);
            let mut connections_guard = connections.lock().await;
            
            let client_index = connections_guard.iter().position(|x| (*x).peer_addr().unwrap() == stream.peer_addr().unwrap()).unwrap();
            connections_guard.remove(client_index);

            break
        }
    }

    Ok(())
    
}

#[async_std::main]
async fn main() -> io::Result<()>{
    let connections: Vec<TcpStream> = vec![];
    let connections = Arc::new(Mutex::new(connections));
    
    let listener = TcpListener::bind("0.0.0.0:28080").await?;
    let mut incoming = listener.incoming();
    let mut con_count = 0;

    println!("use telnet to connect on <host> 28080");

    while let Some(stream) = incoming.next().await {
        con_count += 1;
        println!("{}", con_count);

        let stream = stream?;

        let connections = connections.clone();
        let mut write_permission = connections.lock().await;
        write_permission.push(stream.clone());
        drop(write_permission);

        task::spawn(connection_handler(stream, connections));
    }

    Ok(())
}

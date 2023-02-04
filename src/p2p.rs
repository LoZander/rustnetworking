use std::{net::{IpAddr, TcpStream, ToSocketAddrs, TcpListener, SocketAddr}, io::Write, thread, collections::HashSet};

pub struct P2pErr {text: String}

impl P2pErr {
    pub fn new(message: String) -> Self {
        P2pErr{text: message}
    }
}

impl <T: ToString>From<T> for P2pErr {
    fn from(value: T) -> Self {
        Self::new(value.to_string())
    }
}

pub struct Peer {
    neighbours: HashSet<TcpStream>
}

impl Peer {
    pub fn join<T: ToSocketAddrs>(self,addr: T) -> Result<(),P2pErr> {
        let res = TcpStream::connect(addr);
        if let Ok(mut stream) = res  {
            // LoZander; 2023-02-05; TODO: add connection to neighbours
            thread::spawn(|| handle_connection(stream));
        }

        start_server()?;
        
        Ok(())
    }
}

fn start_server() -> Result<(), P2pErr> {
    let addr: SocketAddr = ([192,168,0,1],5000).into();
    let server = TcpListener::bind(addr)?;

    for stream in server.incoming() {
        let stream = stream?;
        thread::spawn(|| handle_connection(stream));
    }

    Ok(())
}

fn handle_connection(connection: TcpStream) -> Result<(), P2pErr> {
    

    loop {
        let mut buf = [0; 10]; 
        let _ = connection.peek(&mut buf)?;
        
        // LoZander; 2023-02-05; TODO: handle data input
        todo!()
    }
}
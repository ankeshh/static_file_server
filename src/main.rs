use std::fs;
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use clap::{Arg, App};

#[tokio::main]
async fn main(){
    let matches = App::new("Static File Server")
        .version("1.0")
        .author("Ankesh")
        .about("A static file server")
        .arg(
            Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT-NUMBER")
            .takes_value(true)
            .required(false)
            .help("Sets the port number"),
        )
        .arg(
            Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE-NAME")
            .takes_value(true)
            .help("File to be servered")
        )
        .get_matches();
    
    let mut file_server_address = String::from("127.0.0.1:8000");
    let mut file_name = String::from("index.html");

    if let Some(i) = matches.value_of("port"){
        let offset = file_server_address.find('8').unwrap_or(file_server_address.len());
        file_server_address.replace_range(offset.., i);
    }
    if let Some(j) = matches.value_of("file"){
        file_name.replace_range(.., j);
    }

    // Starting server
    println!("Static file server is running on address {}", file_server_address);

    //bind
    let listener = TcpListener::bind(file_server_address).await.unwrap();
    loop{
        let file = file_name.clone();
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move{
            handle_connection(stream, &file).await;
        });
    }

}

async fn handle_connection(mut stream: TcpStream, file_name: &str){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    println!("Connection established");
    
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
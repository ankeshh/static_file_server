use clap::{Arg, App};
use std::net::SocketAddr;
use hyper::server::conn::http1;
use tokio::net::TcpListener;
use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::{Request, Response, Result, StatusCode};

static WELCOME: &[u8] = b"Enter the path in url";
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    
    let mut file_port = String::from("8000");
    let mut file_name = String::from("index.html");

    if let Some(i) = matches.value_of("port"){
        file_port = i.to_string();
    }
    if let Some(j) = matches.value_of("file"){
        file_name = j.to_string();
    }


    // Starting server
    println!("Static file server is running on address {}", file_port);

    //bind
    let addr = SocketAddr::from(([127,0,0,1], file_port.parse::<u16>().unwrap()));
    let listener = TcpListener::bind(addr).await.unwrap();
    loop{
        let file = file_name.clone();
        let (stream, _) = listener.accept().await.unwrap();
        tokio::task::spawn(async move{
            if let Err(err) = 
                http1::Builder::new().serve_connection(stream, service_fn(move |req| handle_connection(req, file.to_string()))).await {
                        eprintln!("Error serving connection: {:?}", err);
                    }
        });
    }

}

async fn handle_connection(req: Request<hyper::body::Incoming>, file: String) -> Result<Response<Full<Bytes>>> {
    let url = format!("{}{}",file,req.uri().path());
    println!("You are currently viewing file: {}", url);
    match req.uri().path() {
        "/" => Ok(Response::builder().status(200).body(Full::new(WELCOME.into())).unwrap()),
        _ => file_send(url).await
    }
}

async fn file_send(filename: String) -> Result<Response<Full<Bytes>>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }
    Ok(not_found())
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOTFOUND.into()))
        .unwrap()
}
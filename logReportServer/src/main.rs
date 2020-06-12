extern crate whoami;
extern crate threadpool;
extern crate base64;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
//use std::path::Path;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str;
use threadpool::ThreadPool;

fn log_write(f: &mut File, s: String) {
    match f.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to log.txt - {}\r\n", why),
        Ok(_) => println!("(timestamp)"),
    }
    match f.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to log.txt - {}\r\n", why),
        Ok(_) => println!("successfully wrote to log.txt\r\n"),
    }
}

fn main() {
    //--------------------------------------------
    //defines
    //--------------------------------------------
    let ipaddress = String::from("127.0.0.1");
    let port = 13303;
    let ipaddr_with_port = format!("{}:{}", ipaddress, port);
    let temp_dir_path = String::from("./temp");
    let report_dir_path = String::from("./report");
    let log_file_name = String::from("log.txt");
    let log_file_path = format!("{}/{}", report_dir_path, log_file_name);
    let stopThread = false;
    let id_log_report = 3777;
    let id_stat_report = 4777;

    //--------------------------------------------
    //1. get logcalhostname
    //--------------------------------------------
    let local_host_name = whoami::hostname();
    println!("hostname = {}", local_host_name);

    //--------------------------------------------
    //2. get osName, win or linux
    //--------------------------------------------
    let mut os_name = whoami::os();
    if os_name.contains("Windows") {
        os_name = String::from("win");
    }
    println!("os name = {}", os_name);

    //--------------------------------------------
    //3. delDir /temp
    //--------------------------------------------
    fs::remove_dir(&temp_dir_path); //no unwrap
    
    //--------------------------------------------
    //4. report dir check, and log.txt write open
    //--------------------------------------------
    fs::create_dir_all(&report_dir_path).unwrap();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut log_file = match File::create(&log_file_path) {
        Err(why) => panic!("couldn't create log.txt - {}", why),
        Ok(file) => file,
    };
    let temp = format!("Server starting\r\nhostname={} , port={}\r\n", local_host_name ,port);
    log_write(&mut log_file, temp);
    // log_write(&mut log_file, String::from("log.txt test2\r\n"));

    //--------------------------------------------
    //5. socketchannel open
    //--------------------------------------------
    let pool = ThreadPool::new(20);
    let listener = TcpListener::bind(&ipaddr_with_port).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("pool execute");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut readbuffer = [0; 128];
    //let mut writebuffer = [0; 128];
    stream.read(&mut readbuffer).unwrap();
    println!("handle_connection");
    //println!("readbuffer={:?}", str::from_utf8(&readbuffer).unwrap());
    let base64_decoded = base64::decode(&readbuffer).unwrap();
    println!("base64 encoded: {:?}", base64_decoded);

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if readbuffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_request(mut stream: TcpStream) {
    unimplemented!()
    // User passes a URL, this page gets called
    // return parsed/filtered webpage
}

fn get_raw_page(url: String) -> String {
    unimplemented!()
    // Make request to website and return the web page
}

fn filter_page(htmlpage: String) -> String {
    unimplemented!()
    // Make request to parser to filter the page and return
    // the filtered page
}

// fn get_domain_ip(url: String) -> ip_addr
fn get_domain_ip(url: String) -> String {
    unimplemented!()
    // Get ip for the domain, likely not needed to be
    // in separate function.

    // Handles 4,5,6,7
}

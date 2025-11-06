use reqwest::blocking::Response;
use serde_json::Value;
use std::collections::HashMap;
use structopt::StructOpt;
use url::{ParseError, Url};

// TODO:
// fix send data in body using form and split to parse into hashmap
// test again take only input.sh and the rest of inner files and put em in the folder like you did for A2
// add the tests from inside the documentation
// add proper comments
#[derive(StructOpt)]
struct Opt {
    url: String,

    #[structopt(short = "X", default_value = "GET")]
    method: String,

    #[structopt(short = "d")]
    data: Option<String>,

    #[structopt(long = "json")]
    json: Option<String>,
}

fn main() {
    let mut opt = Opt::from_args();
    // checking if json flag is set as that means this request must be a POST request
    if let Some(_) = &opt.json {
        opt.method = String::from("POST");
    }
    // print the request info
    println!("Requesting URL: {}", &opt.url);
    println!("Method: {}", &opt.method);
    if let Some(ref data) = opt.data {
        println!("Data: {}", data);
    }
    if let Some(ref json) = opt.json {
        println!("JSON: {}", json);
    }

    // check URL for errors
    let url_ok = check_url(&opt.url);

    // if there is no url parse error then try to send the request out and see what response you get
    if url_ok {
        match &opt.method[..] {
            "GET" => get_request(&opt),
            "POST" => post_request(&opt),
            _ => (),
        }
    }
}

fn check_url(url: &String) -> bool {
    match Url::parse(url) {
        Ok(_) => {
            return true;
        }
        Err(ParseError::RelativeUrlWithoutBase) => {
            println!("Error: The URL does not have a valid base protocol.");
            return false;
        }
        Err(ParseError::RelativeUrlWithCannotBeABaseBase) => {
            println!("Error: The URL does not have a valid base protocol.");
            return false;
        }
        Err(ParseError::SetHostOnCannotBeABaseUrl) => {
            println!("Error: The URL does not have a valid base protocol.");
            return false;
        }
        Err(ParseError::InvalidIpv4Address) => {
            println!("Error: The URL contains an invalid IPv4 address.");
            return false;
        }
        Err(ParseError::InvalidIpv6Address) => {
            println!("Error: The URL contains an invalid IPv6 address.");
            return false;
        }
        Err(ParseError::InvalidPort) => {
            println!("Error: The URL contains an invalid port number.");
            return false;
        }
        Err(e) => {
            println!("Error: The URL has {}", e);
            return false;
        }
    }
}

fn get_request(opt: &Opt) {
    // try to make the get request and print out response
    match reqwest::blocking::get(&opt.url){
        Ok(res)=> print_response(res),
        Err(_)=>println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved."),
    }
}

fn print_response(res: Response) {
    // see if the response has a status code in the 200s else print status code error
    if res.status().is_success() {
        // print out the response
        match res.text() {
            Ok(body) => {
                // parse string into JSON if applicable
                let json_body: Result<Value, _> = serde_json::from_str(&body);
                match json_body {
                    Ok(json) => println!(
                        "Response body (JSON with sorted keys):\n{}",
                        serde_json::to_string_pretty(&json).unwrap()
                    ),
                    // its just a normal response with no JSON
                    Err(_) => println!("Response body:\n{}", body),
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Error: Request failed with status code: {}.", res.status());
    }
}

fn post_request(opt: &Opt) {
    let client = reqwest::blocking::Client::new();
    // check if the request has --json
    let response = if let Some(ref opt_json) = opt.json {
        // parse the string to serde_json::Value
        let value: Value = match serde_json::from_str(opt_json) {
            Ok(v) => v,
            Err(e) => panic!("Invalid JSON: {:?}", e),
        };
        client.post(&opt.url).json(&value).send()
    // check if request has -d
    } else if let Some(ref data) = opt.data {
        // parse the data into hashmap to use the forms in request body
        let mut params = HashMap::new();

        for items in data.split("&") {
            if let Some((key, value)) = items.split_once("=") {
                params.insert(key, value);
            }
        }

        client.post(&opt.url).form(&params).send()
    // send post request with no body
    } else {
        client.post(&opt.url).send()
    };
    // print out response for the post request
    match response{
        Ok(res)=> print_response(res),
        Err(_)=>println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved."),
    }
}

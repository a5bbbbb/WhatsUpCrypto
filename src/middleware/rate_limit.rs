use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}, u128};
use lazy_static::lazy_static;
use axum::{extract::{ConnectInfo, Request}, http::StatusCode, middleware::Next, response::Response};

use crate::utils::utils::get_current_time_millis;


lazy_static! {
    static ref MAP: Arc<Mutex<HashMap<String, RequestInfo>>> = Arc::new(Mutex::new(HashMap::new()));
}

const MAX_REQ: u32 = 10;

#[derive(Debug)]
struct RequestInfo{
    request_count: u32,
    block_time: Option<u128>,
}

pub async fn rate_limit(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next) -> Result<Response, (StatusCode, &'static str)> {


    let ip = addr.ip().to_string();

    let user_agent = req.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    let user_req_id = format!("{}_{}", ip, user_agent);


    match is_allowed_to_proceed(user_req_id) {
        true => {
            let response = next.run(req).await;
            Ok(response)
        }
        false => {
            println!("User is blocked");
            Err((StatusCode::BAD_REQUEST, "You've reached the request limit, please try again later."))
        },
    }
}

fn is_allowed_to_proceed(user_req_id:String) -> bool{
    let mut map = MAP.lock().unwrap();

    let mut user_counter: u32 = match map.get(&user_req_id) {
        Some(val) => {
            if val.block_time != None && get_current_time_millis() <= val.block_time.unwrap() {
                return false;
            }
            val.request_count + 1
        },
        None => { 
            map.insert(user_req_id, RequestInfo{ request_count: 1, block_time: None});
            return true;
        },
        
    };


    let user_block_time = eval_block_time(user_counter);

    if user_block_time != None {
        user_counter = 0;
    }

    map.insert(user_req_id, RequestInfo{ request_count: user_counter,block_time: user_block_time});

    return user_block_time == None;
}

fn eval_block_time(request_count: u32) -> Option<u128> {

    if request_count > MAX_REQ {
        return Some(
            get_current_time_millis() + 60000 //1 min into the future
        );
    }

    return None;
    
}


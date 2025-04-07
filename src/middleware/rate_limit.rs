use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}, u128};
use lazy_static::lazy_static;
use axum::{extract::{ConnectInfo, Request}, http::StatusCode, middleware::Next, response::Response};

use crate::utils::utils::get_current_time_millis;


lazy_static! {
    static ref MAP: Arc<Mutex<HashMap<String, RequestInfo>>> = Arc::new(Mutex::new(HashMap::new()));
}

const MAX_REQ: u32 = 10;
const ONE_MINUTE_SPAN: u128 = 60_000;

#[derive(Debug)]
struct RequestInfo{
    request_count: u32,
    block_time: Option<u128>,
    first_request_time: Option<u128>,
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


    match should_allow_request(user_req_id) {
        true => {
            let response = next.run(req).await;
            Ok(response)
        }
        false => {
            println!("User is blocked");
            Err((StatusCode::TOO_MANY_REQUESTS, "You've reached the request limit, please try again later."))
        },
    }
}

fn should_allow_request(user_req_id:String) -> bool{

    let now = get_current_time_millis();

    let mut map = MAP.lock().unwrap();


    match map.get_mut(&user_req_id) {
        Some(val) => {
            if val.block_time.is_some() && now <= val.block_time.unwrap() {
                return false;
            }

            if now - val.first_request_time.unwrap() <= ONE_MINUTE_SPAN {
                val.request_count += 1;
            }
            else {
                val.request_count = 1;
                val.first_request_time = Some( now );
            }

            val.block_time = eval_block_time(val.request_count, now);

            return val.block_time.is_none()

        },
        None => { 
            map.insert(user_req_id, RequestInfo{ 
                request_count: 1,
                block_time: None,
                first_request_time: Some(now)
            });

            return true;
        },
    }

}

fn eval_block_time(request_count: u32, now: u128) -> Option<u128> {

    if request_count > MAX_REQ {
        return Some(
            now + ONE_MINUTE_SPAN
        );
    }

    return None;
    
}


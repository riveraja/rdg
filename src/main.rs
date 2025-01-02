use redis::Commands;
// use redis::JsonCommands;
// use std::env;
use std::time::{Instant};
use serde_json::json;
use rustop::opts;
use fake::Fake;
use fake::faker::name::raw::*;
use fake::faker::company::raw::*;
use fake::locales::*;
use fake::faker::time::raw::*;
use fake::faker::address::raw::*;
use fake::faker::lorem::en::*;
// use std::collections::BTreeMap;

fn main() {
    let start = Instant::now();
    let (args, _) = opts! {
        version "v0.4.0";
        synopsis "redis-data-generater: A data generator written in Rust for Redis/Valkey";
        opt redis_uri:String="redis://127.0.0.1:6379".to_string(), desc:"The Redis connection string. [default: redis://127.0.0.1:6379]";
        opt count:i32=1000, desc:"Total count of records to be generated. [default: 1000]";
        opt types:Vec<String>, desc:"Types of commands to execute.";
    }.parse_or_exit();
    println!("Hello, world!");
    if args.types.contains(&"set".to_string()) {
        run_set((*args.redis_uri).to_string(), args.count);
    }
    if args.types.contains(&"hset".to_string()) {
        run_hset((*args.redis_uri).to_string(), args.count);
    }
    if args.types.contains(&"json".to_string()) {
        run_json_set((*args.redis_uri).to_string(), args.count);
    }
    let duration = start.elapsed();
    println!("Elapsed time {:?}", duration);
}

fn connect(uri: String) -> redis::Connection {
    let redis_connection_string = format!("{}", uri);
    redis::Client::open(redis_connection_string)
        .expect("Invalid connection string")
        .get_connection()
        .expect("failed to connect")
}

fn run_set(u: String, cnt: i32) {
    let mut conn = connect(u);
    let mut idx: i32 = 0;

    while idx < cnt {
        idx = idx + 1;
        let key = format!("skey:{}", format!("{:0>8}",idx));
        let _: () = conn.set(&key, "qqqqqwwwwwwweeeeeerrrrrttttttyyyyyyyy").unwrap();
    }
}

fn run_hset(u: String, cnt: i32) {
    let mut conn = connect(u);
    // let mut v: BTreeMap<String, String> = BTreeMap::new();
    let mut idx: i32 = 0;

    while idx < cnt {
        idx = idx + 1;
        let key = format!("hkey:{}", format!("{:0>8}",idx));
        let name:String = Name(EN).fake();
        let company:String = CompanyName(EN).fake();

        let _: () = conn.hset_multiple(key, &[(String::from("name"),name),(String::from("company"),company)]).expect("failed");
    }
}

fn run_json_set(u: String, cnt: i32) {
    let mut conn = connect(u);
    let mut idx: i32 = 0;

    while idx < cnt {
        idx = idx + 1;
        let key = format!("jkey:{}",format!("{:0>8}",idx));
        let company:String = CompanyName(EN).fake();
        let industry:String = Industry(EN).fake();
        let startdate:String = Date(EN).fake();
        let city:String = CityName(EN).fake();
        let street:String = StreetName(EN).fake();
        let country:String = CountryName(EN).fake();
        let zip:String = ZipCode(EN).fake();
        let info:Vec<String> = Words(3..8).fake();
        let jsonval = json!({
            "company": company,
            "industry": industry,
            "startdate": startdate,
            "city": city,
            "street": street,
            "country": country,
            "zip": zip,
            "info": info,
        }).to_string();

        let _: () = redis::cmd("JSON.SET").arg(key).arg("$").arg(&jsonval).query(&mut conn).expect("failed");
    }
}
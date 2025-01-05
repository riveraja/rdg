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
use random_number::random;

fn main() {
    let start = Instant::now();
    let (args, _) = opts! {
        version "v0.4.0";
        synopsis "redis-data-generater: A data generator written in Rust for Redis/Valkey";
        opt redis_uri:String="redis://127.0.0.1:6379".to_string(), desc:"The Redis connection string. [default: redis://127.0.0.1:6379]";
        opt count:usize=1000, desc:"Total count of records to be generated. [default: 1000]";
        opt types:Vec<String>, desc:"Types of commands to execute.";
        opt batch:usize=50, desc:"Pipeline size.";
    }.parse_or_exit();
    println!("Generating data");
    if args.types.contains(&"set".to_string()) {
        run_set((*args.redis_uri).to_string(), args.count, args.batch);
    }
    if args.types.contains(&"hset".to_string()) {
        run_hset((*args.redis_uri).to_string(), args.count, args.batch);
    }
    if args.types.contains(&"json".to_string()) {
        run_json_set((*args.redis_uri).to_string(), args.count, args.batch);
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

fn run_set(u: String, cnt: usize, bt: usize) {
    let mut pipe = redis::pipe();
    let mut conn = connect(u);
    let mut idx: usize = 0;
    println!("{} SET", cnt);
    for _i in 0..cnt {
        idx = idx + 1;
        let key = format!("skey:{}", format!("{:0>8}",idx));

        pipe.set(&key, "qqqqqwwwwwwweeeeeerrrrrttttttyyyyyyyy").ignore();

        if (idx % bt) == 0 {
            let () = pipe.query(&mut conn).unwrap();
        }
    }
    let () = pipe.query(&mut conn).unwrap();

}

fn run_hset(u: String, cnt: usize, bt: usize) {
    let mut pipe = redis::pipe();
    let mut conn = connect(u);
    let mut idx: usize = 0;
    println!("{} HSET", cnt);
    for _i in 0..cnt {
        idx = idx + 1;
        let key = format!("hkey:{}", format!("{:0>8}",idx));
        let name:String = Name(EN).fake();
        let company:String = CompanyName(EN).fake();

        pipe.hset_multiple(key, &[(String::from("name"),name),(String::from("company"),company)]).ignore();

        if (idx % bt) == 0 {
            let () = pipe.query(&mut conn).unwrap();
        }
    }
    let () = pipe.query(&mut conn).unwrap();
}

fn run_json_set(u: String, cnt: usize, bt: usize) {
    let mut pipe = redis::pipe();
    let mut conn = connect(u);
    let mut idx: usize = 0;
    println!("{} JSON.SET", cnt);
    for _i in 0..cnt {
        idx = idx + 1;
        let key = format!("jkey:{}",format!("{:0>8}",idx));
        let company:String = CompanyName(EN).fake();
        let industry:String = Industry(EN).fake();
        let startdate:String = Date(EN).fake();
        let city:String = CityName(EN).fake();
        let street:String = StreetName(EN).fake();
        let country:String = CountryName(EN).fake();
        let zip:String = ZipCode(EN).fake();
        let info:Vec<String> = Sentences(3..5).fake();
        let population:usize = random!(50000000,1000000);
        let jsonval = json!({
            "company": company,
            "industry": industry,
            "startdate": startdate,
            "city": city,
            "street": street,
            "country": country,
            "zip": zip,
            "info": info,
            "population": population,
        }).to_string();

        pipe.json_set(key,"$",&jsonval).expect("failed").ignore();

        if (idx % bt) == 0 {
            let () = pipe.query(&mut conn).unwrap();
        }
    }
    let () = pipe.query(&mut conn).unwrap();

}
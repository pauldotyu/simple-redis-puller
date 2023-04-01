use dotenv::dotenv;
use redis::{Commands, RedisResult};
use std::env;

fn main() {
    dotenv().ok();

    // set redis host name
    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");

    // set redis port
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");

    // set redis key
    let redis_key = env::var("REDIS_KEY").expect("REDIS_KEY must be set");

    // set redis list
    let redis_list = env::var("REDIS_LIST").expect("REDIS_LIST must be set");

    // connect to redis
    let client = redis::Client::open(format!(
        "redis://:{}@{}:{}/",
        redis_key, redis_host, redis_port
    ))
    .unwrap();

    // set up connection
    let mut con = client.get_connection().unwrap();

    // get the list length
    let count: usize = con.llen(&redis_list).unwrap();
    
    // while there are items in the list, pop them off and print them
    for i in 0..count {
        let word: RedisResult<String> = con.rpop(&redis_list, None);
        if word.is_err() {
            println!("Error: {:?}", word.err());
            break;
        }
        println!("{} {}", (count - i), word.unwrap());
        
        // sleep for 3 seconds
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    print!("No more words")
}

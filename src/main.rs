use std::sync::{Arc, Mutex};
use std::thread;

use redis::{Client, Commands, Connection, RedisResult};


const KEY: &str = "count";
const CONNECTION_STRING: &str = "redis://redis:6379";


fn update_redis(connection: &mut Connection) -> RedisResult<()> {
    // Function that makes changes to Redis
    let value: i32 = connection.get(KEY).unwrap_or(0);
    println!("Before: {}", value);

    for _ in 0..100 {
        let value: i32 = connection.get(KEY).unwrap_or(0);
        connection.set(KEY, value + 1)?;
    }

    let value: i32 = connection.get(KEY)?;
    println!("After: {}", value);

    Ok(())
}


fn update_redis_with_race_condition(thread_count: usize) -> RedisResult<()> {
    // This will produce a race condition since all threads create their own connection and make changes to Redis at once
    let handles: Vec<_> = (0..thread_count)
        .map(|_| {
            thread::spawn(move || -> RedisResult<()> {
                let client = Client::open(CONNECTION_STRING)?;
                let mut connection = client.get_connection()?;
                update_redis(&mut connection)?;
                Ok(())
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}


fn update_redis_avoiding_race_condition(thread_count: usize) -> RedisResult<()> {
    // This will avoid a race condition since all threads use a common connection guarded by Arc Mutex lock.
    let client = Client::open(CONNECTION_STRING)?;
    let connection = Arc::new(Mutex::new(client.get_connection()?));

    let handles: Vec<_> = (0..thread_count)
        .map(|_| {
            let connection_mutex = Arc::clone(&connection);
            thread::spawn(move || -> RedisResult<()> {
                let mut connection = connection_mutex.lock().unwrap();
                update_redis(&mut connection)?;
                Ok(())
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}


fn main() -> RedisResult<()> {
    let thread_count = 4;

    println!("Running code producing race condition");
    update_redis_with_race_condition(thread_count)?;

    println!("\nRunning code avoiding race condition");
    update_redis_avoiding_race_condition(thread_count)?;

    Ok(())
}

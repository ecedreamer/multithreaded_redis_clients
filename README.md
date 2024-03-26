## Multi-Threaded Redis Client in Rust ##

This Rust application demonstrates how to interact with a Redis server using multiple threads, showcasing both scenarios: one with a race condition and another mitigating it using Arc Mutex.


### Running the Example ###

1. Clone the repository
```
git clone https://github.com/ecedreamer/multithreaded_redis_clients.git

cd multithreaded_redis_clients
```

2. Run docker containers
```
docker compose up --build
```


### Output ###
    multithreaded_redis_clients-rustapp-1  | Running code producing race condition
    multithreaded_redis_clients-rustapp-1  | Before: 0
    multithreaded_redis_clients-rustapp-1  | Before: 0
    multithreaded_redis_clients-rustapp-1  | Before: 0
    multithreaded_redis_clients-rustapp-1  | Before: 0
    multithreaded_redis_clients-rustapp-1  | After: 128
    multithreaded_redis_clients-rustapp-1  | After: 130
    multithreaded_redis_clients-rustapp-1  | After: 147
    multithreaded_redis_clients-rustapp-1  | After: 150
    multithreaded_redis_clients-rustapp-1  | 
    multithreaded_redis_clients-rustapp-1  | Running code avoiding race condition
    multithreaded_redis_clients-rustapp-1  | 
    multithreaded_redis_clients-rustapp-1  | Before: 150
    multithreaded_redis_clients-rustapp-1  | After: 250
    multithreaded_redis_clients-rustapp-1  | Before: 250
    multithreaded_redis_clients-rustapp-1  | After: 350
    multithreaded_redis_clients-rustapp-1  | Before: 350
    multithreaded_redis_clients-rustapp-1  | After: 450
    multithreaded_redis_clients-rustapp-1  | Before: 450
    multithreaded_redis_clients-rustapp-1  | After: 550

### Code Section Explaination ###
The provided Rust code consists of three main functions:

i. update_redis: This function makes changes to a Redis server. It retrieves the current value associated with a key, increments it by one multiple times, and sets the new value back to the Redis server.

ii. update_redis_with_race_condition: This function demonstrates a scenario where multiple threads are spawned, each establishing its own connection to the Redis server. Due to the lack of synchronization, this setup results in a race condition.

iii. update_redis_avoiding_race_condition: This function addresses the race condition issue by ensuring that all threads share a single connection to the Redis server. This is achieved using an Arc Mutex, ensuring that only one thread can access the connection at a time.

### Conclusion ###
This application illustrates the importance of thread safety when interacting with shared resources like a Redis server. By employing synchronization mechanisms such as Arc Mutex, developers can mitigate race conditions and ensure the integrity of their multi-threaded applications.
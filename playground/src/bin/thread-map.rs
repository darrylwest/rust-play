use std::sync::{Arc, RwLock};
use std::thread;
use hashbrown::HashMap;

fn main() {
    // Create a new hash map and wrap it in an Arc and a RwLock
    let map: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));

    // Create some threads to access and modify the map
    let mut handles = Vec::new();
    for i in 0..10 {
        let map_clone = Arc::clone(&map); // Clone the Arc to share the map
        let handle = thread::spawn(move || {
            // Generate some key-value pairs
            let key = format!("key{}", i);
            let value = format!("value{}", i);

            // Get a write lock and insert the pair into the map
            let mut map = map_clone.write().unwrap();
            map.insert(key, value);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Get a read lock and print the map
    let map = map.read().unwrap();
    println!("{:?}", *map);
}


use hashbrown::HashMap;
use log::info;

#[derive(Debug, Clone)]
struct Model {
    id: String,
    name: String,
}

impl Model {
    fn copy(&self) -> Model {
        Model {
            id: self.id.to_string(),
            name: self.name.to_string(),
        }
    }
}

/*
 *
 * Tests
 *
 * single model & list of models
 * write map to file
 * read map from file

 */

fn main() {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("hash tests");

    let count: usize = 6;

    let mut map: HashMap<String, Model> = HashMap::with_capacity(count);
    let list = create_models(count);

    let model: &Model = &list[0];

    info!("model: {:?}", model);
    let key = model.id.to_string();

    for m in &list {
        let id = m.id.to_string();
        let model = m.copy();

        map.insert(id.to_string(), model);
    }

    info!("map: {:?}", map);

    let keys: Vec<String> = map.clone().into_keys().collect();
    info!("keys: {:?}", keys);


    let values: Vec<Model> = map.clone().into_values().collect();
    info!("vals: {:?}", values);

    info!("sizes: map: {}, list: {}", map.len(), list.len());

    if let Some(m) = map.get(&key) {
        info!("get: {:?}", m);
    } else {
        panic!("could not get from map");
    }

    let new_model = Model { id: key.to_string(), name: "new value".to_string() };

    if let Some(old) = map.insert(key, new_model) {
        info!("old: {:?}", old);
    } else {
        panic!("could not update map");
    }
        
    info!("clone: {:?}", &map);


}

fn create_models(count: usize) -> Vec<Model> {
    let mut list: Vec<Model> = Vec::with_capacity(count);
    for idx in 1..=count {
        list.push(Model {
            id: format!("{}", idx + 100),
            name: format!("me-{}", idx),
        });
    }
        
    list
}


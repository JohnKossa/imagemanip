use std::env;
use std::collections::HashMap;

pub fn collect_configs() -> HashMap<String, String>{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut configs: HashMap<String, String> = HashMap::new();
    for arg in args {
        let parts = arg.split('=').collect::<Vec<&str>>();
        if parts.len() as u32 == 2 {
            configs.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    configs
}
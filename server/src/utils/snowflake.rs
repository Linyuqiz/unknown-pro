use lazy_static::lazy_static;
use snowflaked::Generator;
use std::sync::Mutex;
use tracing::info;

lazy_static! {
    static ref GLOBAL_SNOWFLAKE_GENERATOR: Mutex<Generator> = Mutex::new(Generator::new(0));
}

pub fn get_snowflake_id() -> i64 {
    let id = GLOBAL_SNOWFLAKE_GENERATOR.lock().unwrap().generate::<i64>();
    info!("get snowflake id: {}", id);
    id
}

use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String, // shared immutable part
    pub visit_count: Mutex<u32>,       // shared mutable part
}

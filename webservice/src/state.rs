use std::sync::Mutex;

use crate::models::Course;

pub struct AppState {
    pub health_check_response: String,
    //Mutex is a synchronization primitive that can be used to protect shared data
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
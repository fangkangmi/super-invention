use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    item: String,
    vat_incl: bool,
    quantity: i32,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}

// Implement the From trait for the Claims struct
// This will allow us to convert a web::Json<Claims> to a Claims struct
impl From<web::Json<Claims>> for Claims {
    fn from(claims: web::Json<Claims>) -> Self {
        Claims {
            item: claims.item.clone(),
            vat_incl: claims.vat_incl,
            quantity: claims.quantity,
        }
    }
}
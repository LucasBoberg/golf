use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::course::{Course, CourseDTO};
use crate::models::hole::{Hole, HoleDTO};
use crate::models::round::{Round, RoundDTO};
use crate::repository::schema::courses::dsl::*;
use crate::repository::schema::holes::dsl as holes_dsl;
use crate::repository::schema::rounds::dsl::*;

use std::fmt::Error;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use uuid::Uuid;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create db pool.");
        Database { pool }
    }

    pub fn create_round(&self, round: RoundDTO) -> Result<Round, Error> {
        let round = diesel::insert_into(rounds)
            .values(&round)
            .get_result::<Round>(&mut self.pool.get().unwrap())
            .expect("Error saving new round");
        Ok(round)
    }

    pub fn get_rounds(&self) -> Vec<Round> {
        rounds
            .load::<Round>(&mut self.pool.get().unwrap())
            .expect("Error loading rounds")
    }

    pub fn get_round(&self, round_id: &Uuid) -> Option<Round> {
        let round = rounds
            .find(round_id)
            .get_result::<Round>(&mut self.pool.get().unwrap())
            .expect("Error loading round");
        Some(round)
    }

    pub fn create_course(&self, course: CourseDTO) -> Result<Course, Error> {
        let course = diesel::insert_into(courses)
            .values(&course)
            .get_result::<Course>(&mut self.pool.get().unwrap())
            .expect("Error saving new course");
        Ok(course)
    }

    pub fn get_courses(&self) -> Vec<Course> {
        courses
            .load::<Course>(&mut self.pool.get().unwrap())
            .expect("Error loading courses")
    }

    pub fn get_course(&self, course: &Uuid) -> Option<Course> {
        let course = courses
            .find(course)
            .get_result::<Course>(&mut self.pool.get().unwrap())
            .expect("Error loading course");
        Some(course)
    }

    pub fn create_hole(&self, hole: HoleDTO) -> Result<Hole, Error> {
        let hole = diesel::insert_into(holes_dsl::holes)
            .values(&hole)
            .get_result::<Hole>(&mut self.pool.get().unwrap())
            .expect("Error saving new hole");
        Ok(hole)
    }

    pub fn get_holes_by_course_id(&self, course: &Uuid) -> Vec<Hole> {
        holes_dsl::holes
            .filter(holes_dsl::course_id.eq(course))
            .load::<Hole>(&mut self.pool.get().unwrap())
            .expect("Error loading holes")
    }

    pub fn get_hole(&self, hole: &Uuid) -> Option<Hole> {
        let hole = holes_dsl::holes
            .find(hole)
            .get_result::<Hole>(&mut self.pool.get().unwrap())
            .expect("Error loading hole");
        Some(hole)
    }
}

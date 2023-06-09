use actix_web::Error;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

use crate::middlewares::auth::generate_token;
use crate::models::course::{Course, CourseDTO};
use crate::models::hole::{Hole, HoleDTO};
use crate::models::round::{Round, RoundDTO};
use crate::models::user::{AuthResponse, SignInDTO, SignUpDTO, User};
use crate::repository::schema::courses::dsl::*;
use crate::repository::schema::holes::dsl as holes_dsl;
use crate::repository::schema::rounds::dsl::*;
use crate::repository::schema::users::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use uuid::Uuid;

pub struct Database {
    pub pool: DBPool,
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create db pool.");
        pool.get()
            .unwrap() // here we are getting connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Can't run migration");
        Database { pool }
    }

    pub fn register_user(&self, user_dto: SignUpDTO) -> Result<User, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(user_dto.password.as_bytes(), &salt)
            .expect("Error while hashing password")
            .to_string();

        let user_dto = SignUpDTO {
            password: hashed_password,
            ..user_dto
        };

        let user = diesel::insert_into(users)
            .values(&user_dto)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error saving new user");
        Ok(user)
    }

    pub fn sign_in(
        &self,
        secret: String,
        expires_in: i64,
        refresh_secret: String,
        expires_in_refresh: i64,
        sign_in_dto: SignInDTO,
    ) -> Result<AuthResponse, Error> {
        let user = users
            .filter(email.eq(&sign_in_dto.email))
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading user");

        let parsed_hash = PasswordHash::new(&user.password).unwrap();

        Argon2::default()
            .verify_password(sign_in_dto.password.as_bytes(), &parsed_hash)
            .expect("Wrong password or email");

        let token = generate_token(user.id, secret, expires_in);
        let refresh_token = generate_token(user.id, refresh_secret, expires_in_refresh);

        Ok(AuthResponse {
            token,
            refresh_token,
        })
    }

    pub fn get_user(&self, user_id: &Uuid) -> Result<User, Error> {
        let user = users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading user");
        Ok(user)
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

    pub fn delete_round(&self, round_id: &Uuid) -> Result<usize, Error> {
        let count = diesel::delete(rounds.find(round_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting round");
        Ok(count)
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

    pub fn delete_course(&self, course: &Uuid) -> Result<usize, Error> {
        let count = diesel::delete(courses.find(course))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting course");
        Ok(count)
    }

    pub fn update_course(&self, course: &Uuid, course_dto: CourseDTO) -> Result<Course, Error> {
        let course = diesel::update(courses.find(course))
            .set(&course_dto)
            .get_result::<Course>(&mut self.pool.get().unwrap())
            .expect("Error updating course");
        Ok(course)
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

    pub fn delete_hole(&self, hole: &Uuid) -> Result<usize, Error> {
        let count = diesel::delete(holes_dsl::holes.find(hole))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting hole");
        Ok(count)
    }

    pub fn update_hole(&self, hole: &Uuid, hole_dto: HoleDTO) -> Result<Hole, Error> {
        let hole = diesel::update(holes_dsl::holes.find(hole))
            .set(&hole_dto)
            .get_result::<Hole>(&mut self.pool.get().unwrap())
            .expect("Error updating hole");
        Ok(hole)
    }
}

use crate::schema::users;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use diesel::{
    pg,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(pg::Pg))]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    email: String,
    password: String,
}

pub type Conn<'a> = &'a mut PooledConnection<ConnectionManager<PgConnection>>;
pub type PoolConn = Pool<ConnectionManager<PgConnection>>;

#[get("")]
pub async fn list_users(pool: Data<PoolConn>) -> impl Responder {
    let conn: Conn = &mut pool.get().expect("couldn't get db connection from pool");

    let users = users::table
        // .select(users::all_columns)
        .load::<User>(conn)
        .expect("couldn't get users");
    HttpResponse::Ok().json(users)
}

#[post("")]
pub async fn create_user(pool: Data<PoolConn>, user: web::Json<NewUser>) -> impl Responder {
    let conn: Conn = &mut pool.get().expect("couldn't get db connection from pool");

    let new_user = NewUser {
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .expect("couldn't insert user");

    HttpResponse::Created().json(user)
}
pub fn routes() -> actix_web::Scope {
    web::scope("/users")
        .service(list_users)
        .service(create_user)
}

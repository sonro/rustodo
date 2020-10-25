use diesel::pg::PgConnection;
use diesel::Connection;

pub fn get_conn() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use std::sync::Arc;
use std::env::var;

#[cfg(feature = "ssr")]
async fn connect_db() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("DB connection failed");
    db.signin(surrealdb::opt::auth::Root {
        username: var("DB_USER").unwrap(),
        password: var("DB_PASS").unwrap(),
    }).await.expect("Signin failed");
    db.use_ns("namespace").use_db("database").await.expect("Select db failed");
    db
}
#[cfg(feature = "ssr")]
static DB: tokio::sync::OnceCell<Arc<Surreal<Client>>> = tokio::sync::OnceCell::const_new();

#[cfg(feature = "ssr")]
pub async fn get_db() -> Arc<Surreal<Client>> {
    DB.get_or_init(|| async {
        Arc::new(connect_db().await)
    }).await.clone()
}

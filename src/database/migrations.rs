use anyhow::Context;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const DIESEL_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(DIESEL_MIGRATIONS)
        .expect("Couldn't run embedded migrations");
}

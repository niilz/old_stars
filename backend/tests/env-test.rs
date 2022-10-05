#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn url_from_env_var_looks_as_expected() {
        // Given
        let db_user_key = "PG_USER";
        let db_user_dummy = "itseme";
        env::set_var(db_user_key, db_user_dummy);

        let db_pwd_key = "PG_PWD";
        let db_pwd_dummy = "secret";
        env::set_var(db_pwd_key, db_pwd_dummy);

        let db_url_key = "DATABASE_URL";
        let pg_user = env::var(db_user_key).unwrap();
        let pg_pwd = env::var(db_pwd_key).unwrap();
        let db_url_dummy = format!("postgres://{pg_user}:{pg_pwd}@service_name/db_name");
        env::set_var(db_url_key, db_url_dummy);

        let url = env::var(db_url_key).unwrap();
        assert_eq!("postgres://itseme:secret@service_name/db_name", url);
    }
}

use std::time::SystemTime;

use diesel::{PgConnection, insert_into, prelude::*};

use crate::{
    model::{
        history::{History, InsertHistory},
        role::OldStarsRole,
        user::User,
    },
    schema::roles::dsl::{role, roles},
};

use super::error::OldStarsServiceError;

pub const INSERT_HISTORY_FIELDS: [&str; 7] = [
    "user_name",
    "timestamp",
    "beer_count",
    "shot_count",
    "other_count",
    "water_count",
    "cigarette_count",
];
pub fn insert_history_fields_csv_headings() -> String {
    INSERT_HISTORY_FIELDS.join(",") + "\n"
}

pub trait HistoryRepo {
    type Conn;
    // TODO: replace with user_service
    fn get_users(&self, conn: &mut Self::Conn) -> Result<Vec<User>, OldStarsServiceError>;
    fn historize_consumptions(
        &mut self,
        histories: Vec<InsertHistory>,
        conn: &mut Self::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError>;
    fn reset_consumptions(&mut self, conn: &mut Self::Conn) -> Result<(), OldStarsServiceError>;
    fn get_histories(&self, conn: &mut Self::Conn) -> Result<Vec<History>, OldStarsServiceError>;
}

#[derive(Debug, Default)]
pub struct HistoryService<R> {
    pub repo: R,
}

#[derive(Debug, Default)]
pub struct DbHistoryRepo {}

impl HistoryRepo for DbHistoryRepo {
    type Conn = PgConnection;

    fn get_users(&self, conn: &mut Self::Conn) -> Result<Vec<User>, OldStarsServiceError> {
        use crate::schema::old_users::dsl::*;
        let users = old_users
            .inner_join(roles)
            .filter(role.ne(OldStarsRole::Club.to_string()))
            .select(User::as_select())
            .get_results(conn)?;
        Ok(users)
    }

    fn historize_consumptions(
        &mut self,
        histories: Vec<InsertHistory>,
        conn: &mut Self::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError> {
        use crate::schema::history::dsl::*;
        let histories = insert_into(history).values(histories).get_results(conn)?;
        println!("Historized: {histories:?}");
        Ok(histories)
    }

    fn reset_consumptions(&mut self, conn: &mut Self::Conn) -> Result<(), OldStarsServiceError> {
        use crate::schema::old_users::dsl::*;
        diesel::update(old_users)
            .set((
                beer_count.eq(0),
                shot_count.eq(0),
                other_count.eq(0),
                water_count.eq(0),
                cigarette_count.eq(0),
            ))
            .execute(conn)?;
        Ok(())
    }

    fn get_histories(&self, conn: &mut Self::Conn) -> Result<Vec<History>, OldStarsServiceError> {
        use crate::schema::history::dsl::*;
        let histories = history.load::<History>(conn)?;
        Ok(histories)
    }
}

impl<HR: HistoryRepo> HistoryService<HR> {
    pub fn new(repo: HR) -> Self {
        Self { repo }
    }
    pub fn historize_consumptions(
        &mut self,
        conn: &mut HR::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError> {
        let all_users = self.repo.get_users(conn)?;
        let timestamp = SystemTime::now();
        let histories = all_users
            .iter()
            .map(|user| InsertHistory::from((timestamp, user)))
            .collect();
        let written_history = self.repo.historize_consumptions(histories, conn)?;
        self.repo.reset_consumptions(conn)?;
        println!("archive to repo: {written_history:?}");
        Ok(written_history)
    }

    pub fn histories_from_csv(
        &mut self,
        csv: &str,
        conn: &mut HR::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError> {
        let insert_histories = csv_to_history(csv)?;
        self.repo.historize_consumptions(insert_histories, conn)
    }

    pub fn load_histories(
        &self,
        conn: &mut HR::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError> {
        self.repo.get_histories(conn)
    }
}

fn csv_to_history(history_csv: &str) -> Result<Vec<InsertHistory>, OldStarsServiceError> {
    let has_headings = history_csv
        .to_lowercase()
        .starts_with(&insert_history_fields_csv_headings());

    let mut data = history_csv.lines();
    if has_headings {
        data.next();
    }

    data.map(InsertHistory::try_from).collect()
}

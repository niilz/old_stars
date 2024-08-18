use diesel::{insert_into, prelude::*, PgConnection};

use crate::{
    model::{
        history::{History, InsertHistory},
        role::OldStarsRole,
        user::User,
    },
    schema::roles::dsl::{role, roles},
};

use super::error::OldStarsServiceError;

pub trait HistoryRepo {
    type Conn;
    // TODO: replace with user_service
    fn get_drinkers(&self, conn: &mut Self::Conn) -> Result<Vec<User>, OldStarsServiceError>;
    fn historize_drinks(
        &mut self,
        histories: Vec<InsertHistory>,
        conn: &mut Self::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError>;
    fn reset_drinks(&mut self, conn: &mut Self::Conn) -> Result<(), OldStarsServiceError>;
}

#[derive(Debug, Default)]
pub struct HistoryService<R> {
    pub repo: R,
}

pub struct DbHistoryRepo {}

impl HistoryRepo for DbHistoryRepo {
    type Conn = PgConnection;

    fn get_drinkers(&self, conn: &mut Self::Conn) -> Result<Vec<User>, OldStarsServiceError> {
        use crate::schema::old_users::dsl::*;
        let drinkers = old_users
            .inner_join(roles)
            .filter(role.eq(OldStarsRole::User.to_string()))
            .select(User::as_select())
            .get_results(conn)?;
        Ok(drinkers)
    }

    fn historize_drinks(
        &mut self,
        histories: Vec<InsertHistory>,
        conn: &mut Self::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError> {
        use crate::schema::history::dsl::*;
        let histories = insert_into(history).values(histories).get_results(conn)?;
        Ok(histories)
    }

    fn reset_drinks(&mut self, conn: &mut Self::Conn) -> Result<(), OldStarsServiceError> {
        use crate::schema::old_users::dsl::*;
        diesel::update(old_users)
            .set((
                beer_count.eq(0),
                shot_count.eq(0),
                other_count.eq(0),
                water_count.eq(0),
            ))
            .execute(conn)?;
        Ok(())
    }
}

impl<HR: HistoryRepo> HistoryService<HR> {
    pub fn new(repo: HR) -> Self {
        Self { repo }
    }
    pub fn historize_drinks(
        &mut self,
        conn: &mut HR::Conn,
    ) -> Result<Vec<History>, OldStarsServiceError> {
        let all_drinkers = self.repo.get_drinkers(conn)?;
        let histories = all_drinkers.iter().map(InsertHistory::from).collect();
        let written_history = self.repo.historize_drinks(histories, conn)?;
        self.repo.reset_drinks(conn)?;
        Ok(written_history)
    }
}

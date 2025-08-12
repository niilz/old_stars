use super::error::OldStarsServiceError;
use crate::model::user::User;
use crate::schema::old_users::dsl::*;
use diesel::prelude::*;

pub trait ConsumptionRepository {
    type Conn;
    fn update_user(
        &mut self,
        update_user: &User,
        connection: &mut Self::Conn,
    ) -> Result<User, OldStarsServiceError>;
    fn read_user(
        &mut self,
        user_id: i32,
        connection: &mut Self::Conn,
    ) -> Result<User, OldStarsServiceError>;
}

#[derive(Debug, Default)]
pub struct DbConsumptionRepo {}

impl ConsumptionRepository for DbConsumptionRepo {
    type Conn = PgConnection;
    fn update_user(
        &mut self,
        update_user: &User,
        connection: &mut Self::Conn,
    ) -> Result<User, OldStarsServiceError> {
        let updated_user = update_user.save_changes(connection)?;
        Ok(updated_user)
    }

    fn read_user(
        &mut self,
        id: i32,
        connection: &mut Self::Conn,
    ) -> Result<User, OldStarsServiceError> {
        let found_user = old_users.filter(user_id.eq(id)).first(connection)?;
        Ok(found_user)
    }
}

pub struct ConsumptionService<R> {
    pub repo: R,
}

impl<R> ConsumptionService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: ConsumptionRepository> ConsumptionService<R> {
    pub fn add_consumption_to_user(
        &mut self,
        update_id: i32,
        consumption: &str,
        conn: &mut R::Conn,
    ) -> Result<User, OldStarsServiceError> {
        // TODO: Check if adding is allowd according to water:alcohol ratio
        let mut update_user = self.repo.read_user(update_id, conn)?;
        match consumption {
            "beer" => update_user.beer_count += 1,
            "shot" => update_user.shot_count += 1,
            "other" => update_user.other_count += 1,
            "water" => update_user.water_count += 1,
            "cigarette" => update_user.cigarette_count += 1,
            _ => unimplemented!("Other consumption are not supported"),
        };
        self.repo.update_user(&update_user, conn)
    }
}

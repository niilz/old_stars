use std::collections::HashMap;

use backend::{
    model::{
        history::{History, InsertHistory},
        user::User,
    },
    service::history_service::{HistoryRepo, HistoryService},
};

#[derive(Debug, Default)]
pub struct HistoryTestRepo {
    pub users: HashMap<i32, User>,
    pub histories: HashMap<i32, InsertHistory>,
}

impl HistoryTestRepo {
    fn insert(&mut self, histories: Vec<InsertHistory>) {
        for (idx, history) in histories.into_iter().enumerate() {
            self.histories.insert(idx as i32 + 1, history);
        }
    }
}

impl HistoryRepo for HistoryTestRepo {
    type Conn = ();

    fn get_drinkers(
        &self,
        _conn: &mut Self::Conn,
    ) -> Result<Vec<User>, backend::service::error::OldStarsServiceError> {
        Ok(self.users.values().into_iter().map(Clone::clone).collect())
    }

    fn historize_drinks(
        &mut self,
        histories: Vec<backend::model::history::InsertHistory>,
        _conn: &mut Self::Conn,
    ) -> Result<Vec<History>, backend::service::error::OldStarsServiceError> {
        self.insert(histories.clone());
        let histories = self.histories.iter().map(From::from).collect();
        Ok(histories)
    }

    fn reset_drinks(
        &mut self,
        _conn: &mut Self::Conn,
    ) -> Result<(), backend::service::error::OldStarsServiceError> {
        for user in self.users.values_mut() {
            user.beer_count = 0;
            user.shot_count = 0;
            user.other_count = 0;
            user.water_count = 0;
        }
        Ok(())
    }

    fn get_histories(
        &self,
        _conn: &mut Self::Conn,
    ) -> Result<Vec<History>, backend::service::error::OldStarsServiceError> {
        let histories = self.histories.iter().map(History::from).collect();
        Ok(histories)
    }
}

pub fn history_service_mock(user_count: u32) -> HistoryService<HistoryTestRepo> {
    let mut history_service = HistoryService::new(HistoryTestRepo::default());
    for id in 0..user_count {
        let id = id + 1;
        let insert_user = User {
            beer_count: 42,
            shot_count: 43,
            other_count: 44,
            water_count: 45,
            ..Default::default()
        };
        history_service.repo.users.insert(id as i32, insert_user);
    }
    history_service
}

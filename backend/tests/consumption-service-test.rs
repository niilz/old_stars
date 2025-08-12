use std::collections::HashMap;

use backend::{
    model::user::User,
    service::{
        consumption_service::{ConsumptionRepository, ConsumptionService},
        error::OldStarsServiceError,
    },
};

#[derive(Debug, Default)]
struct TestRepo {
    repo: HashMap<i32, User>,
}

impl TestRepo {
    fn insert(&mut self, user: User) {
        self.repo.insert(user.user_id, user);
    }
}

impl ConsumptionRepository for TestRepo {
    type Conn = ();

    fn update_user(
        &mut self,
        update_user: &User,
        _conn: &mut Self::Conn,
    ) -> Result<User, OldStarsServiceError> {
        let _ = self.repo.insert(update_user.user_id, update_user.clone());
        Ok(update_user.clone())
    }

    fn read_user(
        &mut self,
        user_id: i32,
        _connection: &mut Self::Conn,
    ) -> Result<User, OldStarsServiceError> {
        let user = self.repo.get(&user_id).ok_or(OldStarsServiceError::new(
            "Test-Consumption-Repo: ",
            &format!("Could not read user with id: {user_id}"),
        ))?;
        Ok(user.clone())
    }
}

#[test]
fn add_all_the_consumptions() {
    let mut consumption_service_mock = consumption_service_mock();
    let _ = consumption_service_mock.add_consumption_to_user(1, "beer", &mut ());
    let _ = consumption_service_mock.add_consumption_to_user(1, "shot", &mut ());
    let _ = consumption_service_mock.add_consumption_to_user(1, "other", &mut ());
    let _ = consumption_service_mock.add_consumption_to_user(1, "water", &mut ());
    let _ = consumption_service_mock.add_consumption_to_user(1, "cigarette", &mut ());

    let updated_user = consumption_service_mock.repo.read_user(1, &mut ()).unwrap();
    assert_eq!(1, updated_user.beer_count);
    assert_eq!(1, updated_user.shot_count);
    assert_eq!(1, updated_user.other_count);
    assert_eq!(1, updated_user.water_count);
}

fn consumption_service_mock() -> ConsumptionService<TestRepo> {
    let mut consumption_service = ConsumptionService::new(TestRepo::default());
    let dummy_user = dummy_user();
    consumption_service.repo.insert(dummy_user);
    consumption_service
}

fn dummy_user() -> User {
    let mut user = User::default();
    user.user_id = 1;
    user
}

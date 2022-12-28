use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::model::app_user::AppUser;

const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24;

pub struct Session {
    pub user: AppUser,
    pub uuid: String,
    pub exp: Instant,
}

impl Session {
    pub fn new(user: AppUser) -> Self {
        Self {
            user,
            uuid: Uuid::new_v4().to_string(),
            exp: Instant::now() + Duration::from_secs(TWENTY_FOUR_HOURS),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_session() {
        let dummy_user = AppUser {
            id: 1,
            name: "dummy-user".to_string(),
            beer_count: 2,
            shot_count: 2,
            water_count: 1,
        };
        let session = Session::new(dummy_user);

        assert_eq!(session.user.name.to_string(), "dummy-user");
        assert_eq!(36, session.uuid.len());
        let tomorrow = Instant::now() + Duration::from_secs(TWENTY_FOUR_HOURS);
        let one_minute = Duration::from_secs(60);
        let one_minute_earlier = tomorrow - one_minute;
        let one_minute_later = tomorrow + one_minute;
        assert!(session.exp > one_minute_earlier);
        assert!(session.exp < one_minute_later);
    }
}

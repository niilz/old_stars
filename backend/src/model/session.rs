use std::time::{Duration, SystemTime};
use uuid::Uuid;

use super::role::OldStarsRole;

pub const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub user: SessionUser,
    pub uuid: String,
    pub exp: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionUser {
    pub id: i32,
    pub name: String,
    pub role: OldStarsRole,
}

impl Session {
    pub fn new(user: SessionUser) -> Self {
        Self {
            user,
            uuid: Uuid::new_v4().to_string(),
            exp: SystemTime::now() + Duration::from_secs(TWENTY_FOUR_HOURS),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::role::OldStarsRole;
    use crate::model::session::{Session, SessionUser, TWENTY_FOUR_HOURS};
    use std::time::{Duration, SystemTime};

    #[test]
    fn can_construct_session() {
        let dummy_user_name = SessionUser {
            name: "dummy-user".to_string(),
            id: 42,
            role: OldStarsRole::User,
        };
        let session = Session::new(dummy_user_name);

        assert_eq!(session.user.name.to_string(), "dummy-user");
        assert_eq!(36, session.uuid.len());
        let tomorrow = SystemTime::now() + Duration::from_secs(TWENTY_FOUR_HOURS);
        let one_minute = Duration::from_secs(60);
        let one_minute_earlier = tomorrow - one_minute;
        let one_minute_later = tomorrow + one_minute;
        assert!(session.exp > one_minute_earlier);
        assert!(session.exp < one_minute_later);
    }
}

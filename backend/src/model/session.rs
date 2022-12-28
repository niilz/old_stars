use std::time::{Duration, Instant};
use uuid::Uuid;

const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24;

pub struct Session {
    //id: i32,
    user: String,
    uuid: String,
    exp: Instant,
}

impl Session {
    pub fn new(user_name: &str) -> Self {
        Self {
            user: user_name.to_string(),
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
        let dummy_user = "dummy-user";
        let session = Session::new(dummy_user);

        assert_eq!(dummy_user.to_string(), session.user);
        assert_eq!(36, session.uuid.len());
        let tomorrow = Instant::now() + Duration::from_secs(TWENTY_FOUR_HOURS);
        let one_minute = Duration::from_secs(60);
        let one_minute_earlier = tomorrow - one_minute;
        let one_minute_later = tomorrow + one_minute;
        assert!(session.exp > one_minute_earlier);
        assert!(session.exp < one_minute_later);
    }
}

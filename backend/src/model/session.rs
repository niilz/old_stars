// use crate::model::user::User;
// use crate::schema::sessions;
// use diesel::{Associations, Identifiable, Queryable};
// use serde::Deserialize;
use uuid::Uuid;

//#[derive(Deserialize, Queryable, Identifiable, Associations)]
//#[belongs_to(User)]
//#[table_name = "sessions"]
pub struct Session {
    //id: i32,
    user: String,
    uuid: String,
    //value: String,
    //user_id: i32,
}

impl Session {
    fn new(user: &str) -> Self {
        Self {
            user: user.to_string(),
            uuid: Uuid::new_v4().to_string(),
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
    }
}

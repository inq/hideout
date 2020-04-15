use crate::models::User;

pub struct Session {
    user: User,
}

impl Session {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn email(&self) -> &str {
        self.user.email()
    }
}

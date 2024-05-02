use crate::{AppUsers};


impl AppUsers {
    pub fn check_username_exist(&self, username: &str) -> Option<usize> {
        self.users.iter().enumerate().find_map(|(index, user)| {
            if user.cloned_username() == username {
                Some(index)
            } else {
                None
            }
        })
    }
    pub fn check_password(&self, password: &str, index: usize) -> bool {
        if let Some(user) = self.users.get(index) {
            user.verify_password(password)
        } else {
            false
        }
    }
    pub fn check_token(&self, token: &str) -> Option<usize> {
        self.users.iter().enumerate().find_map(|(index, user)| {
            if user.compare_tokens(token)  {
                Some(index)
            } else {
                None
            }
        })
    }
    pub fn get_path(&self, token: &str) ->Option<String>{
        match self.check_token(token){
            None => {
                None
            }
            Some(index) => {
                self.users.get(index).map(|user| user.unique_id.clone())
            }
        }
    }

}
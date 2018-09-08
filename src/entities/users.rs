extern crate uuid;
use self::uuid::Uuid;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct User {
    name: String,
    birthday: u32,
    age: u32,
    taste: Uuid,
    authority: f32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl User {
    pub fn new(name: &str, date: u32) -> User {
        User {
            name: name.to_string(),
            birthday: date,
            age: 0,
            taste: Uuid::new_v4(),
            authority: 0.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.age <= 100
    }

    pub fn step(&mut self) {
        self.age += 1;
        self.authority += 0.1;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_user_init() {
        let user = User::new("Vasyan", 0);
        assert_eq!("Vasyan", user.name);
        assert_eq!(0, user.birthday);
        assert_eq!(0, user.age);
        assert_eq!(4, user.taste.get_version_num());
        assert_eq!(0.0, user.authority);
    }

    #[test]
    fn test_user_step() {
        let mut user = User::new("Vasyan", 0);
        user.step();
        assert_eq!(1, user.age);
        assert_eq!(0.1, user.authority);
    }

    #[test]
    fn test_user_is_alive() {
        let mut user_1 = User::new("Vasyan", 0);
        let mut user_2 = User::new("Vasyan II", 0);
        user_1.age = 10;
        user_2.age = 1000;
        assert_eq!(true, user_1.is_alive());
        assert_eq!(false, user_2.is_alive());
    }
}

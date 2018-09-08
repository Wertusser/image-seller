extern crate uuid;
use self::uuid::Uuid;
use super::users::User;

// TODO: разобраться с лайфтаймом
#[derive(Debug)]
pub struct Image<'a> {
    content: Uuid,
    author: &'a User,
    owner: &'a User
}

impl<'a> Image<'a> {
    pub fn new(author: &'a User) -> Image {
        Image {
            content: Uuid::new_v4(),
            author: author,
            owner: author
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        let my_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
        assert_eq!(4, my_uuid.get_version_num());
    }

    #[test]
    fn test_image_init() {
        let author = User::new("Vasyan", 0);
        let image = Image::new(&author);
        assert_eq!(4, image.content.get_version_num());
        assert_eq!(&author, image.author);
    }
}

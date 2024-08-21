use fake::{faker::internet::en::SafeEmail, Fake};

pub fn fake_email() -> String {
    let email: String = SafeEmail().fake();
    "test-".to_string() + &email
}

use super::Endpoint;

pub struct Held(pub Endpoint);
impl core::ops::Deref for Held {
    type Target = Endpoint;
    fn deref(&self) -> &Endpoint {
        &self.0
    }
}

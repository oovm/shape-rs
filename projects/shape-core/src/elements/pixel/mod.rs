use super::*;

impl Pixel {
    pub fn is_empty(&self, _: &StyleResolver) -> bool {
        self.c.is_empty()
    }
}

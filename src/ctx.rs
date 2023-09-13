#[derive(Clone, Debug)]
pub struct Ctx{
    user_id: u64,
}

// 생성자.
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }    
}

// 속성 접근자.
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
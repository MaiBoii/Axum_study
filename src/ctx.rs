//ctx라고 우리가 http 응답으로부터 원하는 정보만 빼내올 수 있는 추출자를 만들어 놨음.
//여기서는 쿠키 인증을 기반으로 한 유저 id를 빼오는데 사용할 거임.


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
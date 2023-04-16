use va_lib::VAResult;

pub struct VAMicDaemon;

impl VAMicDaemon {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn listen(&self) -> VAResult<()> {
        todo!();
    }
}

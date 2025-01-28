use faststr::FastStr;

pub enum CallbackStatus {
    Continue,
    Abort{reason: FastStr},
}
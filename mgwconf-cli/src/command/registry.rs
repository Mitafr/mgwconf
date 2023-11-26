use lazy_static::lazy_static;

lazy_static! {
    pub static ref AVAILABLE_COMMANDS: [&'static str; 2] = ["GET-SAG", "GET-CERTIFICATES"];
}

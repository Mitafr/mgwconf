use log::info;

#[derive(Debug)]
pub struct Command<'a> {
    options: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new() -> Command<'a> {
        Command { options: vec!["print"] }
    }

    pub fn run(&self) {
        info!("{:#?}", self.options);
        info!("{:#?}", self);
    }
}

use mockall::automock;

#[automock]
pub trait Output {
    fn write(&self, message: &str);
}

pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
}

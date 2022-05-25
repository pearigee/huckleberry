pub fn error<S: Into<String>>(message: S) -> ! {
    panic!("{}", message.into());
}

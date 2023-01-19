use iced::{Application, Error, Sandbox, Settings};

mod view;


  fn main() -> Result<(), Error> {

    view::View::run(Settings::default())

}

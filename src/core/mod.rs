pub mod out;
pub mod react;
pub mod utils;

#[derive(Debug)]
pub enum FileKind {
    ClientComponent,
    ServerAction,
}

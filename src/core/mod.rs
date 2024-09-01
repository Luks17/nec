pub mod out;
pub mod react;

#[derive(Debug)]
pub enum FileKind {
    ClientComponent,
    ServerAction,
}

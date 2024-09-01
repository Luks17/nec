use super::{
    react::{self, React},
    FileKind,
};
use genco::{fmt, Tokens};
use std::fs;

pub fn write_file(name: &str, tokens: Tokens<React>, kind: Option<FileKind>) {
    let file = fs::File::create(format!("output/src/{}", name)).unwrap();
    let mut w = fmt::IoWriter::new(file);

    let fmt = fmt::Config::from_lang::<React>().with_indentation(fmt::Indentation::Space(2));
    let config = match kind {
        None => react::Config::default(),
        Some(FileKind::ClientComponent) => react::Config::default().is_client_component(),
        Some(FileKind::ServerAction) => react::Config::default().is_server_action(),
    };

    tokens
        .format_file(&mut w.as_formatter(&fmt), &config)
        .unwrap();
}

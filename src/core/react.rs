use super::FileKind;
use crate::core::react;
use genco::tokens::ItemStr;
use genco::{fmt, impl_lang};
use relative_path::{RelativePath, RelativePathBuf};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// Tokens container specialization for Rust.
pub type Tokens = genco::Tokens<React>;

impl genco::lang::LangSupportsEval for React {}

impl_lang! {
    /// React language specialization.
    pub React {
        type Config = Config;
        type Format = Format;
        type Item = Import;

        /// Start a string quote.
        fn open_quote(
            out: &mut fmt::Formatter<'_>,
            _config: &Self::Config,
            _format: &Self::Format,
            has_eval: bool,
        ) -> fmt::Result {
            use std::fmt::Write as _;

            if has_eval {
                out.write_char('`')?;
            } else {
                out.write_char('"')?;
            }

            Ok(())
        }

        /// End a string quote.
        fn close_quote(
            out: &mut fmt::Formatter<'_>,
            _config: &Self::Config,
            _format: &Self::Format,
            has_eval: bool,
        ) -> fmt::Result {
            use std::fmt::Write as _;

            if has_eval {
                out.write_char('`')?;
            } else {
                out.write_char('"')?;
            }

            Ok(())
        }

        fn start_string_eval(
            out: &mut fmt::Formatter<'_>,
            _config: &Self::Config,
            _format: &Self::Format,
        ) -> fmt::Result {
            out.write_str("${")?;
            Ok(())
        }

        fn end_string_eval(
            out: &mut fmt::Formatter<'_>,
            _config: &Self::Config,
            _format: &Self::Format,
        ) -> fmt::Result {
            out.write_char('}')?;
            Ok(())
        }

        fn write_quoted(out: &mut fmt::Formatter<'_>, input: &str) -> fmt::Result {
            // Reference: https://mathiasbynens.be/notes/javascript-escapes

            for c in input.chars() {
                match c {
                    // backspace
                    '\u{0008}' => out.write_str("\\b")?,
                    // form feed
                    '\u{0012}' => out.write_str("\\f")?,
                    // new line
                    '\n' => out.write_str("\\n")?,
                    // carriage return
                    '\r' => out.write_str("\\r")?,
                    // horizontal tab
                    '\t' => out.write_str("\\t")?,
                    // vertical tab
                    '\u{0011}' => out.write_str("\\v")?,
                    // null character.
                    '\0' => out.write_str("\\0")?,
                    // Note: only relevant if we were to use single-quoted strings.
                    // '\'' => out.write_str("\\'")?,
                    '"' => out.write_str("\\\"")?,
                    '\\' => out.write_str("\\\\")?,
                    c if !c.is_control() => out.write_char(c)?,
                    c if (c as u32) < 0x100 => {
                        write!(out, "\\x{:02x}", c as u32)?;
                    }
                    c => {
                        write!(out, "\\u{{{:x}}}", c as u32)?;
                    }
                };
            }

            Ok(())
        }

        fn format_file(
            tokens: &Tokens,
            out: &mut fmt::Formatter<'_>,
            config: &Self::Config,
        ) -> fmt::Result {
            let mut imports = Tokens::new();
            Self::imports(&mut imports, tokens, config);
            let format = Format::default();
            imports.format(out, config, &format)?;
            tokens.format(out, config, &format)?;
            Ok(())
        }
    }

    Import {
        fn format(&self, out: &mut fmt::Formatter<'_>, _: &Config, _: &Format) -> fmt::Result {
            let name = match self.kind {
                ImportKind::Named => self.alias.as_ref().unwrap_or(&self.name),
                _ => &self.name,
            };

            out.write_str(name)
        }
    }
}

/// Format state for React.
#[derive(Debug, Default)]
pub struct Format {}

/// Configuration for React.
#[derive(Debug, Default)]
pub struct Config {
    file_kind: Option<FileKind>,
    module_path: Option<RelativePathBuf>,
}

impl Config {
    pub fn with_module_path<M>(self, module_path: M) -> Self
    where
        M: Into<RelativePathBuf>,
    {
        Self {
            module_path: Some(module_path.into()),
            file_kind: self.file_kind,
        }
    }

    pub fn is_client_component(self) -> Self {
        Self {
            module_path: self.module_path,
            file_kind: Some(FileKind::ClientComponent),
        }
    }

    pub fn is_server_action(self) -> Self {
        Self {
            module_path: self.module_path,
            file_kind: Some(FileKind::ServerAction),
        }
    }
}

/// Internal type to determine the kind of import used.
#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum ImportKind {
    Named,
    Default,
    Wildcard,
}

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Import {
    kind: ImportKind,
    module: Module,
    name: ItemStr,
    alias: Option<ItemStr>,
}

impl Import {
    pub fn with_alias<N>(self, alias: N) -> Self
    where
        N: Into<ItemStr>,
    {
        Self {
            kind: ImportKind::Named,
            alias: Some(alias.into()),
            ..self
        }
    }

    pub fn into_default(self) -> Self {
        Self {
            kind: ImportKind::Default,
            alias: None,
            ..self
        }
    }

    pub fn into_wildcard(self) -> Self {
        Self {
            kind: ImportKind::Wildcard,
            alias: None,
            ..self
        }
    }
}

/// A module being imported.
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Module {
    Path(RelativePathBuf),
    Global(ItemStr),
}

impl<'a> From<&'a str> for Module {
    fn from(value: &'a str) -> Self {
        Self::Global(value.into())
    }
}

impl From<String> for Module {
    fn from(value: String) -> Self {
        Self::Global(value.into())
    }
}

impl From<ItemStr> for Module {
    fn from(value: ItemStr) -> Self {
        Self::Global(value)
    }
}

impl React {
    /// Translate imports into the necessary tokens.
    fn imports(out: &mut Tokens, tokens: &Tokens, config: &Config) {
        use genco::prelude::*;

        let mut modules = BTreeMap::<&Module, ResolvedModule<'_>>::new();
        let mut wildcards = BTreeSet::new();

        for import in tokens.walk_imports() {
            match import.kind {
                ImportKind::Named => {
                    let module = modules.entry(&import.module).or_default();

                    module.set.insert(match &import.alias {
                        None => ImportedElement::Plain(&import.name),
                        Some(alias) => ImportedElement::Aliased(&import.name, alias),
                    });
                }
                ImportKind::Default => {
                    let module = modules.entry(&import.module).or_default();
                    module.default_import = Some(&import.name);
                }
                ImportKind::Wildcard => {
                    wildcards.insert((&import.module, &import.name));
                }
            }
        }

        if modules.is_empty() && wildcards.is_empty() && config.file_kind.is_none() {
            return;
        }

        if let Some(kind) = &config.file_kind {
            out.push();
            match kind {
                FileKind::ClientComponent => {
                    quote_in! { *out =>
                        "use client";
                    }
                }
                FileKind::ServerAction => {
                    quote_in! { *out =>
                        "use server";
                    }
                }
            }
            out.line();
        }

        for (module, name) in wildcards {
            out.push();
            quote_in! { *out =>
                import * as $name from $(ref t => render_from(t, config.module_path.as_deref(), module));
            }
        }

        for (name, module) in modules {
            out.push();
            quote_in! { *out =>
                import $(ref tokens => {
                    if let Some(default) = module.default_import {
                        tokens.append(ItemStr::from(default));

                        if !module.set.is_empty() {
                            tokens.append(",");
                            tokens.space();
                        }
                    }

                    if !module.set.is_empty() {
                        tokens.append("{");

                        let mut it = module.set.iter().peekable();

                        while let Some(el) = it.next() {
                            match *el {
                                ImportedElement::Plain(name) => {
                                    tokens.append(name);
                                },
                                ImportedElement::Aliased(name, alias) => {
                                    quote_in!(*tokens => $name as $alias);
                                }
                            }

                            if it.peek().is_some() {
                                tokens.append(",");
                                tokens.space();
                            }
                        }

                        tokens.append("}");
                    }
                }) from $(ref t => render_from(t, config.module_path.as_deref(), name));
            };
        }

        out.line();

        #[derive(Default)]
        struct ResolvedModule<'a> {
            default_import: Option<&'a ItemStr>,
            set: BTreeSet<ImportedElement<'a>>,
        }

        #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum ImportedElement<'a> {
            Plain(&'a ItemStr),
            Aliased(&'a ItemStr, &'a ItemStr),
        }

        fn render_from(t: &mut react::Tokens, module_path: Option<&RelativePath>, module: &Module) {
            quote_in! { *t =>
                $(match (module_path, module) {
                    (_, Module::Global(from)) => $(quoted(from)),
                    (None, Module::Path(path)) => $(quoted(path.as_str())),
                    (Some(module_path), Module::Path(path)) => $(quoted(module_path.relative(path).as_str())),
                })
            }
        }
    }
}

pub fn import<M, N>(module: M, name: N) -> Import
where
    M: Into<Module>,
    N: Into<ItemStr>,
{
    Import {
        kind: ImportKind::Named,
        module: module.into(),
        name: name.into(),
        alias: None,
    }
}

use std::fs::read_to_string;
use std::path::PathBuf;

use anathema_store::smallmap::SmallMap;
use anathema_store::stack::Stack;
use anathema_store::storage::strings::{StringId, Strings};
use anathema_store::storage::Storage;

use crate::blueprints::Blueprint;
use crate::error::{Error, Result};
use crate::statements::eval::Scope;
use crate::statements::parser::Parser;
use crate::statements::{Context, Statements};
use crate::token::Tokens;
use crate::variables::Variables;
use crate::Lexer;

pub(crate) enum SourceKind {
    Path(PathBuf),
    Str(String),
}

impl From<PathBuf> for SourceKind {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

impl From<String> for SourceKind {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<&str> for SourceKind {
    fn from(value: &str) -> Self {
        Self::Str(value.to_string())
    }
}

pub(crate) enum ComponentSource {
    File { path: PathBuf, template: String },
    InMemory(String),
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TemplateComponentId(usize);

impl From<TemplateComponentId> for usize {
    fn from(value: TemplateComponentId) -> Self {
        value.0
    }
}

impl From<usize> for TemplateComponentId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

pub(crate) struct ComponentTemplates {
    dependencies: Stack<TemplateComponentId>,
    components: Storage<TemplateComponentId, String, ComponentSource>,
}

impl ComponentTemplates {
    pub(crate) fn new() -> Self {
        Self {
            dependencies: Stack::empty(),
            components: Storage::empty(),
        }
    }

    pub(crate) fn insert_id(&mut self, name: impl Into<String>) -> TemplateComponentId {
        self.components.push(name.into(), ComponentSource::Empty)
    }

    pub(crate) fn insert(&mut self, ident: impl Into<String>, template: ComponentSource) -> TemplateComponentId {
        let ident = ident.into();
        self.components.insert(ident, template)
    }

    pub(crate) fn load(
        &mut self,
        id: TemplateComponentId,
        globals: &mut Variables,
        slots: SmallMap<StringId, Vec<Blueprint>>,
        strings: &mut Strings,
    ) -> Result<Vec<Blueprint>> {
        if self.dependencies.contains(&id) {
            return Err(Error::CircularDependency);
        }

        self.dependencies.push(id);

        let ret = match self.components.remove(id) {
            Some((key, component_src)) => {
                let template = match &component_src {
                    ComponentSource::File { template, .. } => template,
                    ComponentSource::InMemory(template) => template,
                    ComponentSource::Empty => return Err(Error::MissingComponent),
                };
                let ret = self.compile(&template, globals, slots, strings);
                // This will re-insert the component in the same location
                // as it was removed from since nothing else has
                // written to the component storage since the component
                // was removed.
                let new_id = self.components.insert(key, component_src);
                assert_eq!(id, new_id);
                ret
            }
            _ => return Err(Error::MissingComponent),
        };

        self.dependencies.pop();

        ret
    }

    fn compile(
        &mut self,
        template: &str,
        globals: &mut Variables,
        slots: SmallMap<StringId, Vec<Blueprint>>,
        strings: &mut Strings,
    ) -> Result<Vec<Blueprint>> {
        let tokens = Lexer::new(template, strings).collect::<Result<Vec<_>>>()?;
        let tokens = Tokens::new(tokens, template.len());
        let parser = Parser::new(tokens, strings, template, self);

        let statements = parser.collect::<Result<Statements>>()?;

        let mut context = Context {
            globals,
            components: self,
            strings,
            slots,
        };

        Scope::new(statements).eval(&mut context)
    }

    pub(crate) fn file_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.components.iter().filter_map(|(_, src)| match src {
            ComponentSource::File { path, .. } => Some(path),
            ComponentSource::InMemory(_) => None,
            ComponentSource::Empty => None,
        })
    }

    pub(crate) fn reload(&mut self) -> std::prelude::v1::Result<(), Error> {
        for (_, component) in self.components.iter_mut() {
            match component {
                ComponentSource::File { path, template } => {
                    *template = read_to_string(path)?;
                }
                ComponentSource::InMemory(_) | ComponentSource::Empty => (),
            }
        }
        Ok(())
    }
}

use std::path::Path;

use syntax::ast;
use syntax::attr;

use syntax::visit;
use syntax::visit::Visitor;
use syntax::parse::token::interner_get;

type Module = (~str, Option<Path>);

struct DependenciesContext
{
    path: Path,
    deps: ~[Module],
}

impl Visitor<()> for DependenciesContext
{
    fn visit_item(&mut self, item: @ast::item, _: ())
    {
        let path = self.path.dir_path();
        self.pass_item(path, item);
    }

    fn visit_view_item(&mut self, item: &ast::view_item, _:())
    {
        // let path = self.path.dir_path();
        match item.node
        {
            ast::view_item_extern_mod(id, _, _, _) => {
                let name = interner_get(id.name).to_owned();
                self.pass_extern_mod(name);
            }
            _ => {
                visit::walk_view_item(self, item, ());
            }
        }
    }
}

impl DependenciesContext
{
    fn pass_item(&mut self, path: Path, item: @ast::item)
    {
        match item.node
        {
            ast::item_mod(ref module) => {
                let name = interner_get(item.ident.name).to_owned();
                if !self.pass_mod(path.clone(), name, item)
                {
                    for subitem in module.items.iter()
                    {
                        let subname = interner_get(item.ident.name);
                        self.pass_item(path.join(subname), *subitem);
                    }
                }
            }
            _ => {
                visit::walk_item(self, item, ());
            }
        }
    }

    fn pass_mod(&mut self, path: Path, name: ~str, item: @ast::item) -> bool
    {
        match self.resolve_mod_path(path, name, item)
        {
            Some(path) => {
                self.deps.push((name, Some(path)));
                true
            }
            None => {
                false
            }
        }
    }

    fn pass_extern_mod(&mut self, name: ~str)
    {
        self.deps.push((name, None));
    }

    fn resolve_mod_path(&self, path: Path, name: &str, item: @ast::item) -> Option<Path>
    {
        match attr::first_attr_value_str_by_name(item.attrs, "path")
        {
            Some(d) => Some(path.join(d)),
            None => {
                let default_path_str = name + ".rs";
                let default_path = path.join(default_path_str.as_slice());
                let default_exists = default_path.exists();

                let second_path_str = name + "/mod.rs";
                let second_path = path.join(second_path_str.as_slice());
                let second_exists = second_path.exists();

                match (default_exists, second_exists)
                {
                    (true, false) => Some(default_path),
                    (false, true) => Some(second_path),
                    (false, false) => None,
                    (true, true) => fail!("file for module `{}` found at both {} and {} in {}",
                                            name, default_path_str, second_path_str, self.path.display()),
                }
            }
        }
    }
}

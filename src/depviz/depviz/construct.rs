use std::path::Path;

use syntax::ast;
use syntax::attr;

use syntax::visit;
use syntax::visit::Visitor;
use syntax::parse::token::interner_get;

use depviz::Node;
use depviz::helper;

struct DependenciesContext
{
    path: Path,
    modules: ~[(~str, Path)],
}

impl Visitor<()> for DependenciesContext
{
    fn visit_item(&mut self, item: @ast::item, _:())
    {
        match item.node
        {
            ast::item_mod(_) => {
                let name = interner_get(item.ident.name).to_owned();
                self.resolve_mod(name, item);
            }
            _ => {}
        }
    }
}

impl DependenciesContext
{
    fn resolve_mod(&mut self, name: ~str, item: @ast::item)
    {
        let path = self.resolve_mod_path(name.clone(), item);

        self.modules.push((name.clone(), path));
    }

    fn resolve_mod_path(&self, name: ~str, item: @ast::item) -> Path
    {
        let dir_path = self.path.dir_path();

        match attr::first_attr_value_str_by_name(item.attrs, "name")
        {
            Some(d) => dir_path.join(d),
            None => {
                let default_path_str = name + ".rs";
                let default_path = dir_path.join(default_path_str.as_slice());
                let default_exists = default_path.exists();

                let second_path_str = name + "/mod.rs";
                let second_path = dir_path.join(second_path_str.as_slice());
                let second_exists = second_path.exists();

                match (default_exists, second_exists)
                {
                    (true, false) => default_path,
                    (false, true) => second_path,
                    (false, false) => fail!("file not found for module `{}`", name),
                    (true, true) => fail!("file for module `{}` found at both {} and {}",
                                            name, default_path_str, second_path_str),
                }
            }
        }
    }
}

pub fn construct_crate(name: ~str, path: Path) -> ~Node
{
    let mut root = ~Node::new(name.clone(), path.clone());
    let crate = helper::parse_crate(path.clone());

    let mut ctxt = DependenciesContext {
        path: path.clone(),
        modules: ~[],
    };
    visit::walk_crate(&mut ctxt, &crate, ());

    for module in ctxt.modules.iter()
    {
        let name = module.first();
        let path = module.second();

        let node = construct_crate(name, path);
        root.children.push(node);
    }

    root
}

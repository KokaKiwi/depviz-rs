use std::path::Path;

use extra::sort;

use syntax::visit;

use depviz::Node;
use depviz::helper;
use depviz::visit::DependenciesContext;

pub struct NodeConstructor
{
    priv cache: ~[Path],
}

impl NodeConstructor
{
    pub fn new() -> NodeConstructor
    {
        NodeConstructor {
            cache: ~[],
        }
    }

    pub fn construct_crate(&mut self, name: ~str, path: Path, base: &str) -> ~Node
    {
        self.cache.push(path.clone());

        let mut root = Node::new(name.clone(), path.clone(), base.into_owned());
        let mut ctxt = DependenciesContext {
            path: path.clone(),
            deps: ~[],
        };

        {
            let crate = helper::parse_crate(path.clone());
            visit::walk_crate(&mut ctxt, &crate, ());
        }

        for dep in ctxt.deps.iter()
        {
            let dep_name = dep.first();
            let dep_path = dep.second();

            let node = match dep_path {
                Some(ref dep_path) => {
                    if !self.cache.contains(dep_path)
                    {
                        Some(self.construct_crate(dep_name.clone(), dep_path.clone(), base + "_" + dep_name))
                    }
                    else
                    {
                        None
                    }
                }
                None => {
                    Some(Node::new_extern(dep_name.clone(), base + "_" + dep_name))
                }
            };

            match node
            {
                Some(node) => {
                    root.children.push(node)
                }
                None => {}
            }
        }

        sort::quick_sort(root.children, |a, b| a.name <= b.name);

        root
    }
}

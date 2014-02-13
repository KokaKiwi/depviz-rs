use syntax::ast;
use syntax::parse;

pub fn parse_crate(path: &Path) -> ast::Crate
{
    let sess = parse::new_parse_sess();
    let cfg = create_config();

    parse::parse_crate_from_file(path, cfg, sess)
}

fn create_config() -> ast::CrateConfig
{
    ~[]
}

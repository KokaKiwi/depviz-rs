use syntax::ast;
use syntax::parse;
use syntax::diagnostic;
use std::path::Path;

pub fn parse_crate(path: Path) -> ast::Crate
{
    let demitter = @diagnostic::DefaultEmitter as @diagnostic::Emitter;
    let sess = parse::new_parse_sess(Some(demitter));
    let cfg = create_config();

    let crate = parse::parse_crate_from_file(&path, cfg, sess);

    crate
}

fn create_config() -> ast::CrateConfig
{
    ~[]
}

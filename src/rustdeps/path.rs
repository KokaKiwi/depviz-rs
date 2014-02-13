use std::to_bytes::Cb;
use std::path;

#[deriving(Eq, Clone)]
pub struct Path
{
    parts: ~[~str],
}

impl Path
{
    pub fn new() -> Path
    {
        Path {
            parts: ~[],
        }
    }

    pub fn push<'a>(&'a mut self, part: &str) -> &'a mut Path
    {
        self.parts.push(part.to_owned());

        self
    }

    pub fn pop(&mut self)
    {
        self.parts.pop();
    }

    pub fn name(&self) -> ~str
    {
        self.parts.last().unwrap().to_owned()
    }

    pub fn absolute(&self, base: &Path) -> Path
    {
        let mut p = base.clone();

        for part in self.parts.iter()
        {
            match part.as_slice()
            {
                "super" => {
                    p.pop();
                }
                "self" => {
                }
                part => {
                    p.push(part);
                }
            }
        }

        p
    }

    pub fn to_file(&self, base: &path::Path) -> path::Path
    {
        let mut p = base.clone();

        for part in self.parts.iter()
        {
            p.push(part.as_slice());
        }

        p
    }
}

impl ToStr for Path
{
    fn to_str(&self) -> ~str
    {
        self.parts.connect("::")
    }
}

impl IterBytes for Path
{
    fn iter_bytes(&self, lsb0: bool, f: Cb) -> bool
    {
        let mut result = true;

        for part in self.parts.iter()
        {
            result = result && part.iter_bytes(lsb0, |x| f(x));
        }

        result
    }
}

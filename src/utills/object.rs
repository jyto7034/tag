#[derive(Debug)]
pub struct Tobject {
    pub path: String,
    pub tags: Vec<String>,
}

impl PartialEq for Tobject {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl Tobject {
    pub fn new(path: String, tags: Vec<String>) -> Tobject {
        Tobject { path, tags }
    }

    ///
    /// 정상적인 방법은 아닌듯?
    ///
    pub fn clone(obj: &Tobject) -> Tobject {
        Tobject {
            path: obj.path.clone(),
            tags: obj.tags.clone(),
        }
    }
}

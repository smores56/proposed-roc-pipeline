pub struct Ident<'a> {
    text: &'a str,
    attributes: IdentAttributes,
    problems: IdentProblems,
}

// we'd want to use bitflags
#[derive(Debug, Clone, Copy)]
pub struct IdentAttributes {
    pub bang_suffix: bool,
    pub ignored: bool,
    pub reassignable: bool,
    pub uppercase: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct IdentProblems {
    // TODO
}

impl<'a> Ident<'a> {
    pub fn for_text(&'a self) -> Self {
        todo!()
    }

    pub fn get_raw_text(&'a self) -> &'a str {
        self.text
    }

    pub fn attributes(&self) -> IdentAttributes {
        self.attributes
    }

    pub fn problems(&self) -> IdentProblems {
        self.problems
    }
}

pub enum Default {
    None,
    Value(String),
}

impl Default {
    pub fn serialize(&self) -> String {
        match self {
            Default::None => "".to_owned(),
            Default::Value(value) => format!(" = {}", value),
        }
    }
}
pub enum Docstring {
    // No docs.
    None,

    // A one line /// type comment.
    Simple(String),
}

impl Docstring {
    pub fn serialize(&self, indent: usize) -> String {
        let indent_str = std::iter::repeat(" ").take(indent).collect::<String>();
        match self {
            Docstring::None => "".to_owned(),
            Docstring::Simple(value) => format!("{}/// {}\n", indent_str, value),
        }
    }
}

pub struct MemberVariable {
    dtype: String,
    name: String,
    comment: Docstring,
    default: Default,
}

impl MemberVariable {
    fn serialize(&self, indent: usize) -> String {
        let indent_str = std::iter::repeat(" ").take(indent).collect::<String>();
        return format!(
            "{}{}{} {}{};\n",
            self.comment.serialize(indent),
            indent_str,
            self.dtype,
            self.name,
            self.default.serialize(),
        );
    }
}

pub struct PlainOldStruct {
    docs: Docstring,
    name: String,
    member_variables: Vec<MemberVariable>,
}

impl PlainOldStruct {
    fn serialize(&self) -> String {
        let indent = 4;

        let all_serialized: Vec<String> = self
            .member_variables
            .iter()
            .map(|e| e.serialize(indent))
            .collect();
        let all_vars = all_serialized.join("\n");

        return format!(
            "{}struct {} {{\n{}}};\n\n",
            self.docs.serialize(0),
            self.name,
            all_vars
        );
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_default_none() {
        let d: Default = Default::None;
        assert_eq!("", d.serialize());
    }

    #[test]
    fn test_default_value() {
        let d: Default = Default::Value("123".to_string());
        assert_eq!(" = 123", d.serialize());
    }

    #[test]
    fn test_docstring_none() {
        let d: Docstring = Docstring::None;
        assert_eq!("", d.serialize(4));
    }

    #[test]
    fn test_docstring_value() {
        let d: Docstring = Docstring::Simple("Test variable.".to_string());
        assert_eq!("    /// Test variable.\n", d.serialize(4));
    }

    #[test]
    fn test_member_variable_no_docstring_no_default() {
        let mv = MemberVariable {
            dtype: "uint32_t".to_string(),
            name: "var".to_string(),
            comment: Docstring::None,
            default: Default::None,
        };
        assert_eq!("    uint32_t var;\n", mv.serialize(4));
    }

    #[test]
    fn test_member_variable_docstring_no_default() {
        let mv = MemberVariable {
            dtype: "uint32_t".to_string(),
            name: "var".to_string(),
            comment: Docstring::Simple("This is a test var.".to_string()),
            default: Default::None,
        };
        assert_eq!(
            "    /// This is a test var.\n    uint32_t var;\n",
            mv.serialize(4)
        );
    }

    #[test]
    fn test_member_variable_no_docstring_default() {
        let mv = MemberVariable {
            dtype: "uint32_t".to_string(),
            name: "var".to_string(),
            comment: Docstring::None,
            default: Default::Value("0".to_string()),
        };
        assert_eq!("    uint32_t var = 0;\n", mv.serialize(4));
    }

    #[test]
    fn test_member_variable_docstring_default() {
        let mv = MemberVariable {
            dtype: "uint32_t".to_string(),
            name: "var".to_string(),
            comment: Docstring::Simple("This is a test var.".to_string()),
            default: Default::Value("0".to_string()),
        };
        assert_eq!(
            "    /// This is a test var.\n    uint32_t var = 0;\n",
            mv.serialize(4)
        );
    }

    #[test]
    fn test_empty_struct_no_docstring() {
        let s = PlainOldStruct {
            docs: Docstring::None,
            name: "TestStruct".to_string(),
            member_variables: Vec::new(),
        };
        assert_eq!("struct TestStruct {\n};\n\n", s.serialize());
    }

    #[test]
    fn test_empty_struct_docstring() {
        let s = PlainOldStruct {
            docs: Docstring::Simple("A plain old struct.".to_string()),
            name: "TestStruct".to_string(),
            member_variables: Vec::new(),
        };
        assert_eq!(
            "/// A plain old struct.\nstruct TestStruct {\n};\n\n",
            s.serialize()
        );
    }

    #[test]
    fn test_one_var_no_docstring() {
        let s = PlainOldStruct {
            docs: Docstring::None,
            name: "TestStruct".to_string(),
            member_variables: vec![MemberVariable {
                dtype: "uint32_t".to_string(),
                name: "var".to_string(),
                comment: Docstring::Simple("This is a test var.".to_string()),
                default: Default::Value("0".to_string()),
            }],
        };
        assert_eq!(
            "struct TestStruct {\n    /// This is a test var.\n    uint32_t var = 0;\n};\n\n",
            s.serialize()
        );
    }
}

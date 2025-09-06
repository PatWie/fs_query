#[cfg(test)]
mod tests {
    use super::super::{CodeParser, get_language};
    use super::super::symbol::SymbolKind;
    use super::super::cpp_visitor::CppVisitor;
    use std::collections::HashSet;

    #[test]
    fn test_cpp_function_extraction() {
        let cpp_code = r#"
void simple_function() {}

namespace ns {
    void namespaced_function() {}
}

void dd::test() {}
"#;

        let visitor = CppVisitor::new();
        let language = get_language("test.cpp").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(cpp_code, visitor).unwrap();
        
        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        
        assert_eq!(functions.len(), 3);
        
        let simple = functions.iter().find(|f| f.name == "simple_function").unwrap();
        let namespaced = functions.iter().find(|f| f.name == "namespaced_function").unwrap();
        let dd_test = functions.iter().find(|f| f.name == "dd::test").unwrap();
        
        assert_eq!(simple.start_line, 2);
        assert_eq!(namespaced.start_line, 5);
        assert_eq!(dd_test.start_line, 8);
    }

    #[test]
    fn test_cpp_class_extraction() {
        let cpp_code = r#"
class TestClass {
public:
    void method() {}
};

struct TestStruct {
    int member;
};
"#;

        let visitor = CppVisitor::new();
        let language = get_language("test.cpp").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(cpp_code, visitor).unwrap();
        
        let classes: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Class)
            .collect();
        
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "TestClass");
        assert_eq!(classes[0].start_line, 2);
    }

    #[test]
    fn test_cpp_variable_extraction() {
        let cpp_code = r#"
int global_var = 42;
const float PI = 3.14f;

void function() {
    int local_var = 10;
    double another_var;
}
"#;

        let visitor = CppVisitor::new();
        let language = get_language("test.cpp").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(cpp_code, visitor).unwrap();
        
        let variables: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Variable)
            .collect();
        
        assert!(variables.len() >= 2); // At least global_var and PI
        
        let global_var = variables.iter().find(|v| v.name.contains("global_var")).unwrap();
        let pi = variables.iter().find(|v| v.name.contains("PI")).unwrap();
        
        assert_eq!(global_var.start_line, 2);
        assert_eq!(pi.start_line, 3);
    }

    #[test]
    fn test_cpp_filtering() {
        let cpp_code = r#"
class TestClass {};
void test_function() {}
int global_var = 42;
"#;

        let language = get_language("test.cpp").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        // Test filtering for functions only
        let mut function_filter = HashSet::new();
        function_filter.insert(SymbolKind::Function);
        
        let symbols = parser.extract_symbols(cpp_code, "test.cpp", Some(function_filter)).unwrap();
        
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].kind, SymbolKind::Function);
        assert_eq!(symbols[0].name, "test_function");
    }

    #[test]
    fn test_cpp_line_numbers() {
        let cpp_code = r#"void first_function() {}

class TestClass {
    void method() {}
};

void last_function() {}"#;

        let visitor = CppVisitor::new();
        let language = get_language("test.cpp").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(cpp_code, visitor).unwrap();
        
        let first_func = symbols.iter().find(|s| s.name == "first_function").unwrap();
        let test_class = symbols.iter().find(|s| s.name == "TestClass").unwrap();
        let last_func = symbols.iter().find(|s| s.name == "last_function").unwrap();
        
        assert_eq!(first_func.start_line, 1);
        assert_eq!(test_class.start_line, 3);
        assert_eq!(last_func.start_line, 7);
    }
}

#[cfg(test)]
mod tests {
    use super::super::{SymbolKind, CodeParser, get_language};
    use super::super::go_visitor::GoVisitor;
    use std::collections::HashSet;

    #[test]
    fn test_go_function_extraction() {
        let go_code = r#"
package main

func main() {
    println("Hello, World!")
}

func add(a, b int) int {
    return a + b
}

func (p Point) Distance() float64 {
    return math.Sqrt(p.X*p.X + p.Y*p.Y)
}
"#;

        let visitor = GoVisitor::new();
        let language = get_language("test.go").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(go_code, visitor).unwrap();
        
        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        
        assert!(functions.len() >= 2);
        
        let main_func = functions.iter().find(|f| f.name == "main").unwrap();
        let add_func = functions.iter().find(|f| f.name == "add").unwrap();
        
        assert_eq!(main_func.start_line, 4);
        assert_eq!(add_func.start_line, 8);
    }

    #[test]
    fn test_go_struct_extraction() {
        let go_code = r#"
package main

type Point struct {
    X, Y float64
}

type Rectangle struct {
    Width  float64
    Height float64
}
"#;

        let visitor = GoVisitor::new();
        let language = get_language("test.go").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(go_code, visitor).unwrap();
        
        let structs: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Struct)
            .collect();
        
        assert_eq!(structs.len(), 2);
        
        let point = structs.iter().find(|s| s.name == "Point").unwrap();
        let rectangle = structs.iter().find(|s| s.name == "Rectangle").unwrap();
        
        assert_eq!(point.start_line, 4);
        assert_eq!(rectangle.start_line, 8);
    }

    #[test]
    fn test_go_variable_extraction() {
        let go_code = r#"
package main

var globalVar = 42
var pi = 3.14159

func main() {
    var localVar int = 10
}
"#;

        let visitor = GoVisitor::new();
        let language = get_language("test.go").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(go_code, visitor).unwrap();
        
        let variables: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Variable)
            .collect();
        
        assert!(variables.len() >= 2);
        assert!(variables.iter().any(|v| v.name == "globalVar"));
        assert!(variables.iter().any(|v| v.name == "pi"));
    }

    #[test]
    fn test_go_filtering() {
        let go_code = r#"
package main

type TestStruct struct {
    field int
}

func testFunction() {}

var testVar = 42
"#;

        let language = get_language("test.go").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let mut struct_filter = HashSet::new();
        struct_filter.insert(SymbolKind::Struct);
        
        let symbols = parser.extract_symbols(go_code, "test.go", Some(struct_filter)).unwrap();
        
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].kind, SymbolKind::Struct);
        assert_eq!(symbols[0].name, "TestStruct");
    }
}

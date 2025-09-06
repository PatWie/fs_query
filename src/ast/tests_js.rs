#[cfg(test)]
mod tests {
    use super::super::{SymbolKind, CodeParser, get_language};
    use super::super::js_visitor::JsVisitor;
    use std::collections::HashSet;

    #[test]
    fn test_js_function_extraction() {
        let js_code = r#"
function hello() {
    console.log("Hello");
}

const add = function(a, b) {
    return a + b;
};

function multiply(x, y) {
    return x * y;
}
"#;

        let visitor = JsVisitor::new();
        let language = get_language("test.js").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(js_code, visitor).unwrap();
        
        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        
        assert!(functions.len() >= 2); // hello and multiply (add might not be detected as function_declaration)
        
        let hello = functions.iter().find(|f| f.name == "hello").unwrap();
        let multiply = functions.iter().find(|f| f.name == "multiply").unwrap();
        
        assert_eq!(hello.start_line, 2);
        assert_eq!(multiply.start_line, 10);
    }

    #[test]
    fn test_js_class_extraction() {
        let js_code = r#"
class Calculator {
    constructor() {
        this.value = 0;
    }
    
    add(x) {
        return this.value + x;
    }
}

class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}
"#;

        let visitor = JsVisitor::new();
        let language = get_language("test.js").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(js_code, visitor).unwrap();
        
        let classes: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Class)
            .collect();
        
        assert_eq!(classes.len(), 2);
        
        let calculator = classes.iter().find(|c| c.name == "Calculator").unwrap();
        let point = classes.iter().find(|c| c.name == "Point").unwrap();
        
        assert_eq!(calculator.start_line, 2);
        assert_eq!(point.start_line, 12);
    }

    #[test]
    fn test_js_variable_extraction() {
        let js_code = r#"
const PI = 3.14;
let counter = 0;
var name = "test";
"#;

        let visitor = JsVisitor::new();
        let language = get_language("test.js").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(js_code, visitor).unwrap();
        
        let variables: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Variable)
            .collect();
        
        assert_eq!(variables.len(), 3);
        assert!(variables.iter().any(|v| v.name == "PI"));
        assert!(variables.iter().any(|v| v.name == "counter"));
        assert!(variables.iter().any(|v| v.name == "name"));
    }

    #[test]
    fn test_js_filtering() {
        let js_code = r#"
class TestClass {}
function testFunction() {}
const testVar = 42;
"#;

        let language = get_language("test.js").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let mut function_filter = HashSet::new();
        function_filter.insert(SymbolKind::Function);
        
        let symbols = parser.extract_symbols(js_code, "test.js", Some(function_filter)).unwrap();
        
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].kind, SymbolKind::Function);
        assert_eq!(symbols[0].name, "testFunction");
    }
}

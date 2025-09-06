#[cfg(test)]
mod tests {
    use super::super::{CodeParser, get_language};
    use super::super::symbol::SymbolKind;
    use super::super::python_visitor::PythonVisitor;
    use std::collections::HashSet;

    #[test]
    fn test_python_function_extraction() {
        let python_code = r#"
def hello_world():
    print("Hello, World!")

def add(a, b):
    return a + b

def multiply(x, y):
    return x * y
"#;

        let visitor = PythonVisitor::new();
        let language = get_language("test.py").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(python_code, visitor).unwrap();
        
        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        
        assert_eq!(functions.len(), 3);
        
        let hello_world = functions.iter().find(|f| f.name == "hello_world").unwrap();
        let add = functions.iter().find(|f| f.name == "add").unwrap();
        let multiply = functions.iter().find(|f| f.name == "multiply").unwrap();
        
        assert_eq!(hello_world.start_line, 2);
        assert_eq!(add.start_line, 5);
        assert_eq!(multiply.start_line, 8);
    }

    #[test]
    fn test_python_class_extraction() {
        let python_code = r#"
class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, x):
        return self.value + x

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
"#;

        let visitor = PythonVisitor::new();
        let language = get_language("test.py").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(python_code, visitor).unwrap();
        
        let classes: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Class)
            .collect();
        
        assert_eq!(classes.len(), 2);
        
        let calculator = classes.iter().find(|c| c.name == "Calculator").unwrap();
        let point = classes.iter().find(|c| c.name == "Point").unwrap();
        
        assert_eq!(calculator.start_line, 2);
        assert_eq!(point.start_line, 9);
    }

    #[test]
    fn test_python_mixed_symbols() {
        let python_code = r#"
def main():
    print("Hello")

class TestClass:
    def method(self):
        pass

def another_function():
    return 42
"#;

        let visitor = PythonVisitor::new();
        let language = get_language("test.py").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(python_code, visitor).unwrap();
        
        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        let classes: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Class)
            .collect();
        
        assert_eq!(functions.len(), 3); // main, method, another_function
        assert_eq!(classes.len(), 1); // TestClass
    }

    #[test]
    fn test_python_filtering() {
        let python_code = r#"
class TestClass:
    pass

def test_function():
    pass

class AnotherClass:
    def method(self):
        pass
"#;

        let language = get_language("test.py").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        // Test filtering for classes only
        let mut class_filter = HashSet::new();
        class_filter.insert(SymbolKind::Class);
        
        let symbols = parser.extract_symbols(python_code, "test.py", Some(class_filter)).unwrap();
        
        assert_eq!(symbols.len(), 2);
        assert!(symbols.iter().all(|s| s.kind == SymbolKind::Class));
        assert!(symbols.iter().any(|s| s.name == "TestClass"));
        assert!(symbols.iter().any(|s| s.name == "AnotherClass"));
    }

    #[test]
    fn test_python_line_numbers() {
        let python_code = r#"def first_function():
    pass

class TestClass:
    def method(self):
        pass

def last_function():
    return True"#;

        let visitor = PythonVisitor::new();
        let language = get_language("test.py").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(python_code, visitor).unwrap();
        
        let first_func = symbols.iter().find(|s| s.name == "first_function").unwrap();
        let test_class = symbols.iter().find(|s| s.name == "TestClass").unwrap();
        let last_func = symbols.iter().find(|s| s.name == "last_function").unwrap();
        
        assert_eq!(first_func.start_line, 1);
        assert_eq!(test_class.start_line, 4);
        assert_eq!(last_func.start_line, 8);
    }

    #[test]
    fn test_python_nested_functions() {
        let python_code = r#"
def outer_function():
    def inner_function():
        return "nested"
    return inner_function()

class OuterClass:
    def outer_method(self):
        def inner_function():
            return "nested in method"
        return inner_function()
"#;

        let visitor = PythonVisitor::new();
        let language = get_language("test.py").unwrap();
        let mut parser = CodeParser::new(language).unwrap();
        
        let symbols = parser.parse_with_visitor(python_code, visitor).unwrap();
        
        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        
        // Should find outer_function, inner_function (x2), and outer_method
        assert!(functions.len() >= 3);
        assert!(functions.iter().any(|f| f.name == "outer_function"));
        assert!(functions.iter().any(|f| f.name == "outer_method"));
    }
}

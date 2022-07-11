use std::collections::HashMap;

enum Type {
    Int(i32),
    Float(f32),
}

struct VarInfo {
    val: Type,
}

struct SymbolTable {
    ht_stack: Vec<HashMap<String, VarInfo>>,
}

impl SymbolTable {
    fn init() -> SymbolTable {
        SymbolTable {
            ht_stack: vec![HashMap::new()],
        }
    }

    fn insert(mut self, lineno: usize, id: &str, info: VarInfo) {
        // if variable is already in scope raise an error
        if self.ht_stack[self.ht_stack.len() - 1].contains_key(id) {
            panic!("Variable {} already declared at line {}", id, lineno);
        }
        let n = self.ht_stack.len();
        self.ht_stack[n - 1].insert(id.to_owned(), info);
    }
}

struct Parser {

}

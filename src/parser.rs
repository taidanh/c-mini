use std::collections::HashMap;

use crate::scanner::{Scanner, Token};
use crate::ast::*;

// struct VarInfo {
//     val: Type,
//     node_type: NodeType,
//     id_type: IDTypes,
// }

#[derive(PartialEq)]
enum IDTypes {
    IO,
    Var,
}

struct NewLabelGenerator {
    counter: usize
}

impl NewLabelGenerator {
    fn new() -> Self {
        Self { counter: 0}
    }

    fn mk_new_label(&mut self) -> String {
        self.counter += 1;
        format!("label{}", self.counter - 1)
    }
}

struct NewNameGenerator {
    counter: usize,
    new_names: Vec<String>,
}

impl NewNameGenerator {
    fn new() -> Self {
        Self { counter: 0, new_names: Vec::new() }
    }

    fn mk_new_name(&mut self) -> String {
        let new_name = format!("_new_name{:?}", self.counter);
        self.counter += 1;
        self.new_names.push(new_name.clone());
        new_name
    }
}

struct VRAllocator {
    counter: usize
}

impl VRAllocator {
    fn new() -> Self {
        Self {
            counter: 0
        }
    }

    fn mk_new_vr(&mut self) -> String {
        self.counter += 1;
        format!("vr{:?}", self.counter - 1)
    }

    fn declare_variables(self) -> Vec<String> {
        let mut ret = Vec::new();
        for i in 0..self.counter {
            ret.push(format!("virtual_reg vr{:?}", i));
        }

        ret
    }
}

struct SymbolTableData {
    id_type: IDTypes,
    data_type: Type,
    new_name: String,
}

impl SymbolTableData {
    pub fn new(&self, id_type: IDTypes, data_type: Type, new_name: String) -> Self {
        Self {
            id_type,
            data_type,
            new_name,
        }
    }

    pub fn get_id_type(&self) -> IDTypes {
        self.id_type
    }

    pub fn get_data_type(&self) -> Type {
        self.data_type
    }

    pub fn get_new_name(&self) -> String {
        self.new_name
    }
}

struct SymbolTable {
    ht_stack: Vec<HashMap<String, SymbolTableData>>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            ht_stack: vec![HashMap::new()],
        }
    }

    fn insert(mut self, lineno: usize, id: &str, info: SymbolTableData) {
        // if variable is already in scope raise an error
        if self.ht_stack[self.ht_stack.len() - 1].contains_key(id) {
            panic!("Variable {} already declared at line {}", id, lineno);
        }
        let n = self.ht_stack.len();
        self.ht_stack[n - 1].insert(id.to_owned(), info);
    }

    fn lookup(&self, id: Option<String>) -> Option<&SymbolTableData> {
        if id.is_none() {
            panic!("ID is none")
        }
        let id_val = id.unwrap();
        for ht in self.ht_stack.iter().rev() {
            match ht.get(&id_val) {
                Some(v) => return Some(v),
                None => continue
            };
        }
        return None;
    }

    fn get_new_name(&mut self) {

    }
}

fn linearize_expr(node: Node) -> Option<Vec<String>> {
    todo!()
}

struct Parser {
    nng: NewNameGenerator,
    symbol_table: SymbolTable,
    scanner: Scanner,
    vra: VRAllocator,
    nlg: NewLabelGenerator,
    uf: usize,
    to_match: Option<Token>,
}

impl Parser {
    fn new(scanner: Scanner) -> Self {
        Self {
            nng: NewNameGenerator::new(),
            symbol_table: SymbolTable::new(),
            scanner,
            vra: VRAllocator::new(),
            nlg: NewLabelGenerator::new(),
            uf: 1,
            to_match: None,
        }
    }

    fn parser_exception(&self, lineno: usize, to_match: Option<String>, tokens: &[&str]) {
        panic!("Parser error on line {lineno}\nExpected one of: {:?}, found {:?}", tokens, to_match.unwrap_or("Null".to_owned()));
    }

    fn eat(&mut self, check: &'static str) {
        self.eat_maybe(Some(check.to_owned()));
    }

    fn eat_maybe(&mut self, check: Option<String>) {
        let to_match = self.to_match.as_ref().and_then(|t| { Some(t.0.to_owned()) });
        if to_match != check {
            panic!("Expected {:?} but got {:?}", check, self.to_match);
        }
        self.to_match = self.scanner.token();
    }

    fn combine(&self, mut v: Option<Vec<String>>, e: Option<String>) -> Option<Vec<String>> {
        if e.is_none() { return v; }

        match v {
            Some(v1) => {
                if e.is_some() {
                    v1.push(e.unwrap());
                }
                Some(v1)
            },
            None => Some(Vec::from([e.unwrap()])),
        }
    }

    fn get_token_id(&self) -> Option<String> {
        self.to_match.as_ref().and_then(|t| { Some(t.0.to_owned()) })
    }

    fn check_tok_list(&self, id: &Option<String>, toks: &[&str]) -> bool {
        if id.is_some() && toks.iter().any(|&i| i == id.unwrap()) {
            return true;
        }
        false
    }

    fn check_tok(&self, id: &Option<String>, tok: &str) -> bool {
        id.is_some() && id.unwrap() == tok
    }

    fn assign_vrs(&mut self, node: &mut Node) {
        for i in 0..node.children.len() {
            self.assign_vrs(&mut node.children[i]);
        }

        let new_vr = self.vra.mk_new_vr();
        node.set_vr(&new_vr);
    }

    fn parse(&mut self) {
        self.to_match = self.scanner.token();
        self.eat_maybe(None);
        let p = self.parse_function();

        p
    }

    fn parse_function(&mut self) {
        self.parse_function_header();
        self.eat("LBRACE");
        let p = self.parse_statement_list();
        self.eat("RBRACE");

        p
    }

    fn parse_function_header(&mut self) {
        self.eat("VOID");
        // let func_name = self.to_match.and_then(|t| { Some(t.1.to_owned()) }).unwrap();
        let _func_name: String;
        if self.to_match.is_some() {
            _func_name = self.to_match.as_ref().unwrap().1.to_owned();
        } else {
            panic!("Error parsing function name");
        }
        self.eat("ID");
        self.eat("LPAR");
        let _args = self.parse_args_list();
        self.eat("RPAR");
    }

    fn parse_args_list(&mut self) -> Option<Vec<String>> {
        let mut token_id = self.get_token_id();
        if self.check_tok(&token_id, "RPAR") {
            return None;
        }
        let arg = self.parse_arg();
        token_id = self.get_token_id();
        if self.check_tok(&token_id, "RPAR") {
            match arg {
                Some(a) => return Some(Vec::from([a])),
                None    => return None,
            }
        }
        let arg_list = self.parse_args_list();
        return self.combine(arg_list, arg);
    }

    fn parse_arg(&mut self) -> Option<String> {
        let token_id = self.get_token_id();
        let data_type: Type;
        let data_type_str: String;
        if self.check_tok(&token_id, "FLOAT") {
            self.eat("FLOAT");
            data_type = Type::Float(0.0);
            data_type_str = "float".to_string();
        } else if self.check_tok(&token_id, "INT") {
            self.eat("INT");
            data_type = Type::Int(0);
            data_type_str = "int".to_owned();
        } else {
            self.parser_exception(111, self.get_token_id(), &["INT", "FLOAT"]);
        }

        self.eat("AMP");
        let id_name = self.to_match.expect("Error: ID name not found").1;
        self.eat("ID");
        // self.symbol_table.insert(id_name, IDTypes.IO, data_type);
        return None;
    }

    fn parse_statement_list(&mut self) -> Option<Vec<String>> {
        let token_id = self.get_token_id();
        let p0: Option<String>;
        let p1: Option<Vec<String>>;
        if self.check_tok_list(&token_id, &["INT", "FLOAT", "ID", "IF", "LBRACE", "FOR"]) {
            p0 = self.parse_statement();
            p1 = self.parse_statement_list();
        }

        if self.check_tok(&token_id, "RBRACE") {
            return None;
        } else {
            panic!("Unknown token found at the end of statement list");
        }
    }

    fn parse_statement(&self) -> Option<Vec<String>> {
        let token_id = self.get_token_id();
        let p: Option<Vec<String>>;
        if self.check_tok_list(&token_id, &["INT", "FLOAT"]) {
            self.parse_declaration_statement();
            return None;
        } else if self.check_tok(&token_id, "ID") {
            p = self.parse_assignment_statement();
            return p;
        } else if self.check_tok(&token_id, "IF") {
            p = self.parse_if_else_statement();
            return p;
        } else if self.check_tok(&token_id, "LBRACE") {
            p = self.parse_block_statement();
            return p;
        } else if self.check_tok(&token_id, "FOR") {
            p = self.parse_for_statement();
            return p;
        }

        self.parser_exception(111, token_id, &["FOR", "IF", "LBRACE", "INT", "FLOAT", "ID"]);
        None
    }

    fn parse_declaration_statement(&self) {
        let token_id = self.get_token_id();
        let id_name = self.to_match.as_ref().and_then(|t| { Some(t.1.to_owned()) });
        if self.check_tok(&token_id, "INT") {
            self.eat("INT");
            let id_name = self.to_match.as_ref().and_then(|t| { Some(t.1.to_owned()) });
        } else if self.check_tok(&token_id, "FLOAT") {
            self.eat("FLOAT");
            let id_name = self.to_match.as_ref().and_then(|t| { Some(t.1.to_owned()) });
        } else {
            self.parser_exception(111, token_id, &["INT", "FLOAT"]);
        }

        // self.symbol_table.insert(id_name, IDTypes.VAR, Type.Int);
        self.eat("ID");
        self.eat("SEMI");
        return;
    }

    fn parse_assignment_statement(&self) -> Option<Vec<String>> {
        let p = self.parse_assignment_statement_base();
        self.eat("SEMI");
        p
    }

    fn parse_assignment_statement_base(&self) -> Option<Vec<String>> {
        let id_name = self.to_match.as_ref().and_then(|t| { Some(t.1.to_owned()) });
        let id_data = self.symbol_table.lookup(id_name).unwrap_or_else(|| {
            panic!("Id {:?} has not been created", id_name);
        });
        self.eat("ID");
        self.eat("ASSIGN");

        let mut ast = self.parse_expr();
        type_inference(ast);

        if id_data.get_data_type() != ast.val_type {
            match id_data.get_data_type() {
                Type::Int(i) => ast = Node::grow_ast(NodeType::FloatToInt, id_data.get_data_type(), ast),
                Type::Float(f) => ast = Node::grow_ast(NodeType::IntToFloat, id_data.get_data_type(), ast),
            }
        }

        self.assign_vrs(&mut ast);
        let program = linearize_expr(ast);

        let assignment_program: String;
        match id_data.get_id_type() {
            IDTypes::Var => {
                assignment_program = format!("{:?} = {:?};", id_data.get_new_name(), ast.vr);
            },
            IDTypes::IO => {
                let op = if id_data.get_data_type().is_int() {
                    "vr2int".to_owned()
                  } else {
                    "vr2float".to_owned()
                  };
                assignment_program = format!("{:?} = {:?}({:?});", id_name, op, ast.vr);
            },
        }

        self.combine(program, Some(assignment_program))
    }

    fn parse_if_else_statement(&self) -> Option<Vec<String>> {
        self.eat("IF");
        self.eat("LPAR");

        let ast = self.parse_expr();
        type_inference(ast);
        self.assign_vrs(&mut ast);
        let p0 = linearize_expr(ast);

        let zero = self.vra.mk_new_vr();
        let else_label = self.nlg.mk_new_label();
        let end_label = self.nlg.mk_new_label();
        let i0 = format!("{:?} = int2vr(0);")
    }

    fn parse_block_statement(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn parse_for_statement(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn parse_expr(&self) -> Node {
        todo!()
    }
}

fn type_inference(ast: Node) {
    todo!()
}

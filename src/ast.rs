use core::fmt;

#[derive(PartialEq)]
pub enum Type {
    Int(i32),
    Float(f32),
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Int => write!(f, "i"),
            Float => write!(f, "f")
        }
    }
}

impl Type {
    pub fn is_int(&self) -> bool {
        matches!(*self, Self::Int(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(*self, Self::Float(_))
    }
}

#[derive(PartialEq)]
pub enum NodeType {
    Num,
    VarID,
    IOID,
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    Lt,
    IntToFloat,
    FloatToInt,
    IntToVR,
    FloatToVR,
    Leaf,
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num        => write!(f, "2vr"),
            VarID      => write!(f, ""),
            IOID       => write!(f, "2vr"),
            Add        => write!(f, "add"),
            Sub        => write!(f, "sub"),
            Mult       => write!(f, "mult"),
            Div        => write!(f, "div"),
            Eq         => write!(f, "eq"),
            Lt         => write!(f, "lt"),
            IntToFloat => write!(f, "vr_int2float"),
            FloatToInt => write!(f, "vr_float2int"),
            IntToVR    => write!(f, "int2vr"),
            FloatToVR  => write!(f, "float2vr"),
            Leaf       => write!(f, "Leaf Node"),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub val_type: Type,
    pub children: Vec<Node>,
    pub vr: Option<String>,
}

impl Node {
    pub fn new(node_type: NodeType, val_type: Type) -> Self {
        Self {
            node_type,
            val_type,
            children: Vec::new(),
            vr: None,
        }
    }

    pub fn set_vr(&mut self, vr: &String) {
        self.vr = Some(vr.to_owned());
    }

    pub fn three_addr_code(self) -> String {
        let args: String;
        // figure out the args first to reduce branches
        match self.children.len() {
            0 => args = "".to_string(),
            1 => args = format!("({:?})", self.children[0]),
            2 => args = format!("({:?},{:?})", self.children[0], self.children[1]),
            _ => panic!("This node has {:?} children, not 1 or 2", self.children.len())
        }

        // VarID, IOID, and Num are special cases
        // because int and float are printed differently
        match self.node_type {
            NodeType::VarID => "".to_string(),
            NodeType::IOID  |
            NodeType::Num   => {
                match self.val_type {
                    Type::Int(_)   => format!("{:?} = int{:?}{:?}", self.vr, self.node_type, args),
                    Type::Float(_) => format!("{:?} = float{:?}{:?}", self.vr, self.node_type, args),
                }
            }
            _ => format!("{:?} = {:?}{:?}{:?};", self.vr, self.node_type, self.val_type, args)
        }
    }

    pub fn grow_ast(node_type: NodeType, val_type: Type, ast: Node) -> Self {
        Self {
            node_type,
            val_type,
            children: Vec::from([ast]),
            vr: None,
        }
    }
}

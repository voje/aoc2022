use std::fmt;

type NodeID = usize;

pub trait Node {
    fn get_parent(&self) -> Option<NodeID>;
    fn set_parent(&mut self, nid: NodeID);
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
}

#[derive(Debug)]
struct Dir {
    parent: Option<NodeID>,
    name: String,
    children: Vec<NodeID>
}

impl Dir {
    fn add_child(&mut self, nidx: NodeID) {
        self.children.push(nidx); 
    }
    fn new(name: String) -> Dir {
        Dir {
            parent: None,
            name,
            children: vec![],
        }
    }
}

impl Node for Dir {
    fn set_parent(&mut self, nid: NodeID) {
        self.parent = Some(nid);
    }
    fn get_parent(&self) -> Option<NodeID> {
        self.parent
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Debug)]
struct File {
    parent: Option<NodeID>,
    size: i64,
    name: String,
}

impl Node for File {
    fn set_parent(&mut self, nid: NodeID) {
        self.parent = Some(nid);
    }
    fn get_parent(&self) -> Option<NodeID> {
        self.parent
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

impl File {
    fn new(size: i64) -> File {
        File {
            parent: None,
            size,
            name: "FILENAME".to_string(),
        }
    }
}

struct FileSystem {
    current_node_idx: NodeID,
    mem_arena: Vec<Box<dyn Node>>,
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "root: {}\ncurrent_node: {}\nmem_arena: {}",
            self.mem_arena[0].get_name(),
            self.mem_arena[self.current_node_idx].get_name(),
            "MEM_ARENA",
        ) 
    } 
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            current_node_idx: 0,
            mem_arena: vec![],
        };
        // Add root node
        let root = Dir::new("/".to_string());
        fs.add_node(root); 
        fs
    }

    fn add_node(&mut self, n: impl Node + 'static) -> usize {
        self.mem_arena.push(Box::new(n));
        self.mem_arena.len() - 1
    }


//    fn walk(&self, node_idx: NodeID, depth: usize) {
//        let padding = " ".to_string().repeat(depth); 
//        let node = &self.mem_arena[node_idx];
//        // println!("{}{:?}", padding, node);
//        match &node.node_type {
//            NodeType::Dir(name, children) => {
//                println!("{}Dir: {}", padding, name);
//                for c_idx in children {
//                    println!("HA");
//                    self.walk(*c_idx, depth + 1);
//                }
//            }
//            NodeType::File(size) => {
//                println!("{}File: {}", padding, size);
//            }
//        }
//    }
}

fn part1(s: &str) {
    // todo
}

#[cfg(test)]
const DATA_1: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn day07_file_system() {
    let mut fs = FileSystem::new();
    let a = Dir::new("a".to_string());
    fs.add_node(a);

//     fs.add_node(NodeType::File(22));
//     fs.add_node(NodeType::Dir(Dir{
//         name: "one".to_string(),
//         children: vec![],
//     });
// 
//     fs.walk(0, 0);

    println!("{}", fs)
}

#[test]
fn day07_part1() {
    // todo
}

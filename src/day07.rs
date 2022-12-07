type NodeID = usize;

enum NodeType {
    Dir(String, Vec<NodeID>),
    File(i64),
}

struct Node {
    parent: Option<NodeID>,
    node_type: NodeType
}

struct FileSystem {
    current_node_idx: NodeID,
    mem_arena: Vec<Node>
}

impl FileSystem {
    fn new() -> FileSystem {
        // Root node
        let n = Node {
            parent: None,
            node_type: NodeType::Dir("/".to_string(), vec![]),
        };
        FileSystem {
            mem_arena: vec![n],
            current_node_idx: 0,
        }
    }

    // The current node is the 'parent' node
    fn add_node(&mut self, node_type: NodeType) {
        let node = Node {
            parent: Some(self.current_node_idx),
            node_type,
        };
        self.mem_arena.push(node);
        self.current_node_idx = self.mem_arena.len() - 1;
    }

    fn walk() {
        
    }
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
    let fs = FileSystem::new();
    fs.add_node(NodeType::File(22));

    fs.walk(0);
}

#[test]
fn day07_part1() {
    // todo
}

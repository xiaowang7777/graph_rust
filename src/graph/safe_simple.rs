use std::error::Error;
use std::io::Read;
use crate::graph::{ExampleError, Graph, GraphPath};

pub struct Node {
    // 节点编号
    v: usize,
    // 和当前节点相连的节点列表
    list: Vec<isize>,
}

pub struct SimpleGraph {
    // 顶点数
    v: usize,
    // 边数
    e: usize,
    list: Vec<Node>,
}

impl SimpleGraph {
    fn new(cap: usize) -> Self {
        let mut list = Vec::with_capacity(cap);
        for i in 0..cap {
            list.push(Node::new(i));
        }

        SimpleGraph {
            v: cap,
            e: 0,
            list,
        }
    }

    fn new_with_reader(reader: Box<dyn Read>) -> Self {
        todo!()
    }
}

impl Graph for SimpleGraph {
    type NodeItem = isize;
    type DFSItem = SimpleDFSGraphPath;
    type BFSItem = SimpleBFSGraphPath;

    fn v(&self) -> usize {
        self.v
    }

    fn e(&self) -> usize {
        self.e
    }

    fn add_edge(&mut self, v: usize, e: usize) {
        self.list.get_mut(v).map(|node| {
            let mut l = 0;

            while let Some(&i) = node.list.get(l) {
                if i == e as isize {
                    return;
                }
                l += 1;
            }
            node.list.push(e as isize);
        });

        self.list.get_mut(e).map(|node| {
            node.list.push(v as isize);
        });

        self.v += 1;
    }

    fn adj(&self, v: usize) -> Vec<Self::NodeItem> {
        self.list.get(v).map(|node| {
            node.list.clone()
        }).take().unwrap()
    }

    fn to_dfs(self, v: usize) -> Self::DFSItem {
        SimpleDFSGraphPath::new(self.list, v)
    }

    fn to_bfs(self, v: usize) -> Self::BFSItem {
        todo!()
    }
}

pub struct SimpleDFSGraphPath {
    point: usize,
    marked: Vec<bool>,
    path: Vec<isize>,
    list: Vec<Node>,
}

impl SimpleDFSGraphPath {
    fn new(list: Vec<Node>, point: usize) -> Self {
        let len = list.len();
        let mut marked = Vec::with_capacity(len);
        let mut path = Vec::with_capacity(len);
        for _ in 0..len {
            marked.push(false);
            path.push(-1);
        }

        let mut p = SimpleDFSGraphPath {
            point,
            marked,
            path,
            list,
        };

        p.dfs(point);

        p
    }

    fn dfs(&mut self, point: usize) {
        self.marked.get_mut(point).map(|b| {
            if *b {
                return;
            }
            *b = true;
        });
        if let Some(node) = (&mut self.list).get_mut(point) {
            let mut len: usize = 0;
            while let Some(&mut n) = (&mut node.list).get_mut(len) {
                len += 1;
                if n < 0 {
                    continue;
                }
                if let Some(&true) = self.marked.get(n as usize) {
                    continue;
                }
                self.path.get_mut(n as usize).map(|p| {
                    *p = point as isize;
                });
                self.dfs(n as usize);
            }
        }
    }
}

impl GraphPath for SimpleDFSGraphPath {
    fn has_path_to(&self, to: usize) -> bool {
        *self.marked.get(to).unwrap()
    }

    fn path_to(&self, to: usize) -> Result<Vec<usize>, Box<dyn Error>> {
        if !self.has_path_to(to) {
            Err(Box::new(ExampleError))
        } else {
            let mut res = Vec::new();
            let mut start = to;
            let path = &self.path;
            while let Some(&i) = path.get(start) {
                if i < 0 {
                    break;
                }
                res.push(start);
                start = i as usize;
            }
            res.push(self.point);
            Ok(res)
        }
    }
}


pub struct SimpleBFSGraphPath {
    point: usize,
    marked: *mut Vec<bool>,
    path: *mut Vec<isize>,
    list: Vec<Node>,
}

impl Node {
    fn new(v: usize) -> Self {
        Node {
            v,
            list: Vec::new(),
        }
    }
}

mod test {
    use crate::graph::{Graph, GraphPath};
    use crate::graph::simple::SimpleGraph;

    #[test]
    fn test_dfs() {
        let mut g = SimpleGraph::new(6);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 5);
        let dfs = g.to_dfs(1);
        assert!(dfs.has_path_to(5));
        let path = dfs.path_to(5).unwrap();
        assert_eq!(path, vec![5, 0, 1])
    }
}
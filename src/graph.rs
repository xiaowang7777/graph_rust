use std::any::TypeId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;

pub trait Graph {
    type NodeItem;
    type DFSItem;
    type BFSItem;

    // 顶点数
    fn v(&self) -> usize;
    // 边数
    fn e(&self) -> usize;
    // 新增v和e的连接
    fn add_edge(&mut self, v: usize, e: usize);
    // 返回和节点v相连的节点
    fn adj(&self, v: usize) -> Vec<Self::NodeItem>;
    // 将图以节点v为基准，使用深度优先算法进行计算，并转移所有权
    fn to_dfs(self, v: usize) -> Self::DFSItem;
    // 将图以节点v为基准，使用广度优先算法进行计算，并转移所有权
    fn to_bfs(self, v: usize) -> Self::BFSItem;
}

pub trait GraphPath {
    fn has_path_to(&self, to: usize) -> bool;
    fn path_to(&self, to: usize) -> Result<Vec<usize>, Box<dyn Error>>;
}

pub struct ExampleError ;

impl Debug for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ExampleError{

}

mod simple;
mod safe_simple;
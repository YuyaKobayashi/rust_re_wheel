use std::rc::Rc;
use std::iter::Iterator;

enum Error {
    NodeExist,
}

type NodeRef<T> = Rc<Node<T>>;
type OptChildNode<T> = Option<NodeRef<T>>;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    left: OptChildNode<T>, // TODO: mutable
    right: OptChildNode<T>, // TODO: mutable
}

#[derive(Debug)]
struct Tree<T> {
    root: NodeRef<T>,
}

#[derive(Debug)]
struct DFSIter<'a, T> {
    tree: &'a Tree<T>,
    exp_stack: Vec<NodeRef<T>>,
}

impl<T> Node<T> where T: Copy {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None
        }
    }

    pub fn get_val(&self) -> T {
        self.val
    }
    pub fn set_val(&mut self, val: T) {
        self.val = val;
    }

    fn _assign(&mut self, node: Self, is_left: bool) -> Result<(), Error> {
        let dst = if is_left {&mut self.left} else {&mut self.right};

        if dst.is_none() {
            *dst = Some(Rc::new(node));
            Result::Ok(())
        } else {
            Result::Err(Error::NodeExist)
        }
    }
    pub fn assign_left(&mut self, node: Self) -> Result<(), Error> {
        self._assign(node, true)
    }
    pub fn assign_right(&mut self, node: Self) -> Result<(), Error> {
        self._assign(node, false)
    }
}

impl<T> Tree<T> {
    pub fn new(root: Node<T>) -> Self {
        Self {
            root: Rc::new(root)
        }
    }

    pub fn iter(&self) -> DFSIter<'_, T> {
        let mut it = DFSIter {
            tree: self,
            exp_stack: vec![],
        };
        it.exp_stack.push(it.tree.root.clone());
        it
    }
}

impl<T> Iterator for DFSIter<'_, T> {
    type Item = NodeRef<T>;

    fn next(self: &mut Self) -> Option<Self::Item> {
        let node = self.exp_stack.pop()?;

        if let Some(ref c) = node.right {
            self.exp_stack.push(c.clone());
        }
        if let Some(ref c) = node.left {
            self.exp_stack.push(c.clone());
        }

        Some(node)
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn build_tree() {
        let tree = super::Tree::new(super::Node::new(1));

        print!("{tree:#?}\n", tree=tree);
    }
}


use std::fmt::Debug;

#[derive(Debug)]
pub struct AST<T: Debug> {
    left: Option<Box<AST<T>>>,
    center: T,
    right: Option<Box<AST<T>>>
}


impl<T> AST<T>
where T: Debug {
    pub fn new(val: T) -> Self {
        Self {
            left: None,
            center: val, 
            right: None
        }
    }

    fn new_full(left: T, center: T, right: T ) -> Self {
        Self {
            left: Some(
                Box::new(
                    AST::new(left)
                )
            ),
            center: center, 
            right : Some(
                Box::new(
                    AST::new(right)
                )
            )
        }
    }

    pub fn full_self(left: Self, center: T, right: Self) -> Self {
        Self {
            left: Some(
                Box::new(
                    left
                )
            ),
            center,
            right: Some(
                Box::new(
                    right
                )
            )
        }
    }

    pub fn left(& self) -> Option<&Box<AST<T>>> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&Box<AST<T>>> {
        self.right.as_ref()
    }

    pub fn left_mut(&mut self) -> &mut Option<Box<AST<T>>> {
        &mut self.left
    }

    pub fn right_mut(&mut self) -> &mut Option<Box<AST<T>>> {
        &mut self.right
    }
    
    pub fn view(&self) -> &T {
        &self.center
    }
    
    pub fn edit(&mut self) -> &mut T {
        &mut self.center
    }

    pub fn preorder_traverse(&self) {
        println!("{:?}", self.view());

        if self.left.is_some() {
            AST::preorder_traverse(self.left.as_ref().unwrap())
        } 

        if self.right.is_some() {
            AST::preorder_traverse(self.right.as_ref().unwrap())
        }
    }
}




mod tests{
    use super::AST;

    fn insert(ast: &mut AST<usize>, val: usize){
        let mut new = ast;
        
        loop {
            if val <= new.center {
                if new.left().is_some() {
                    new = new.left_mut().as_mut().unwrap()
                } else {
                    *new.left_mut() = Some(
                        Box::new(
                            AST::new(
                                val
                            )
                        )
                    )
                }
            }else {
                if new.right().is_some() {
                    new = new.right_mut().as_mut().unwrap()
                } else {
                    *new.right_mut() = Some(
                        Box::new(
                            AST::new(
                                val
                            )
                        )
                    )
                }
            }
        }
    }

    fn binary_tree() {
        let mut bt = AST::new(10_usize);
        let br = [10, 5, 15, 3, 7, 12, 20, 1, 4, 6, 8, 11, 14, 18, 25];
        for i in br { 
            insert(&mut bt, i)
        }
        bt.preorder_traverse()
    }
    
}

use std::fmt::Debug;

#[derive(Debug,Clone)]
pub struct AST<T: Debug> {
    center: T,
    children: Vec<AST<T>>
}


impl<T> AST<T>
where T: Debug {
    pub fn new(val: T) -> Self {
        Self {
            center: val, 
            children: Vec::with_capacity(0)
        }
    }

    pub fn new_with_children(center: T, mut children: Vec<Self>) -> Self {
        children.shrink_to_fit();
        Self {
            center,
            children
        }
    }

    pub fn unary(center: T, right: Self) -> Self {
        let mut children = Vec::with_capacity(1);
        children.push(right);
        Self {
            center: center,
            children
        }
    }

    pub fn full_self(left: Self, center: T, right: Self) -> Self {
        let mut children = Vec::with_capacity(2);
        children.push(left);
        children.push(right);
        Self {
            center,
            children
        }
    }

    pub fn left(& self) -> Option<&AST<T>> {
        Some(&self.children[0])
    }

    pub fn right(&self) -> Option<&AST<T>> {
        Some(&self.children[1])
    }

    pub fn children(&self) -> &Vec<AST<T>> {
        &self.children
    }
    
    pub fn view(&self) -> &T {
        &self.center
    }
    
    // pub fn edit(&mut self) -> &mut T {
    //     &mut self.center
    // }

}
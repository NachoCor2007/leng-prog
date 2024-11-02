use std::rc::Rc;

#[derive(Debug)]
struct Node<T>(T, Stack<T>);

#[derive(Debug)]
struct Stack<T>(Option<Rc<Node<T>>>);

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        match &self.0 {
            None => {Stack(None)}
            Some(r) => {Stack(Some(r.clone()))}
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack(None)
    } // An empty Stack

    fn push(&self, value: T) -> Self {
        let n: Node<T> = Node(value, self.clone());
        let r: Rc<Node<T>> = Rc::new(n);

        Stack(Some(r))
    }

    fn peek(&self) -> Option<&T> {
        match &self.0 {
            None => { None }
            Some(r) => { Some(&r.0) }
        }
    }

    /// If the stack is empty returns `None`
    /// else returns Some(tuple) where tuple contains a reference to the value
    /// on the top of the stack plus the modified `Stack`
    fn pop(&self) -> Option<(&T, Stack<T>)> {
        match &self.0 {
            None => {None}
            Some(r) => {
                let v: &T = &r.0;
                let s: &Stack<T> = &r.1;

                Some((v, s.clone()))
            }
        }
    }

    fn is_empty(&self) -> bool {
        match &self.0 {
            None => {true}
            Some(_) => {false}
        }
    }
}
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub struct Heap<T> {
    n: isize,
    min: Option<Rc<Node<T>>>,
}

pub struct InputData<T> {
    key: isize,
    value: T,
    is_marked: bool,
    children: Vec<InputData<T>>,
}

impl<T> Heap<T> {
    // TODO
    pub fn set_n(&mut self, n: isize) {
        self.n = n;
    }

    pub fn construct(input: Vec<InputData<T>>) -> Self {
        let mut heap = Self::new();
        for x in input {
            let node = Node::new(x.key, x.value, x.is_marked);
            node.construct_child(x.children);
            heap.insert_node(node);
        }

        return heap;
    }

    pub fn new() -> Self {
        Heap { n: 0, min: None }
    }

    pub fn get_min(&self) -> Option<Rc<Node<T>>> {
        if let Some(min) = &self.min {
            Some(Rc::clone(min))
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: isize, value: T, is_marked: bool) {
        let node = Node::new(key, value, is_marked);
        self.insert_node(Rc::clone(&node));
        self.n += 1;
    }

    pub fn insert_node(&mut self, node: Rc<Node<T>>) {
        self.min = match &self.min {
            Some(min) => {
                min.concatenate(Rc::clone(&node));
                if node.get_key() < min.get_key() {
                    Some(node)
                } else {
                    Some(Rc::clone(min))
                }
            }
            None => Some(node),
        };
    }
    pub fn minimum(&self) -> isize {
        *self.get_min().unwrap().key.borrow()
    }

    pub fn union(&mut self, mut heap: Self) {
        if let Some(heap_min) = heap.get_min() {
            if let Some(self_min) = self.get_min() {
                if self_min.get_key() > heap_min.get_key() {
                    self.min = Some(Rc::clone(&heap_min));
                }
                self_min.concatenate(Rc::clone(&heap_min));
            } else {
                self.min = Some(heap_min);
            }
            heap.min = None;
        }
    }

    pub fn extract_min(&mut self) -> Option<Rc<Node<T>>> {
        if let Some(z) = self.get_min() {
            if let Some(z_child) = z.get_child() {
                self.get_min().unwrap().concatenate(Rc::clone(&z_child));
            }
            let z_right = z.get_right().unwrap();
            z.remove();
            if Rc::ptr_eq(&z, &z_right) {
                self.min = None;
            } else {
                self.min = Some(z_right);
                self.consolidate();
            }
            self.n -= 1;
            return Some(z);
        }
        None
    }

    fn D(&self) -> usize {
        let n = self.n as f64;
        let log = n.log2();
        log as usize
    }

    fn consolidate(&mut self) {
        let D = self.D();
        // while letを抜けるbreakがうまく動かないので余分に空間を確保
        let mut A = vec![None as Option<Rc<Node<T>>>; &D + 2];

        if let Some(min) = self.get_min() {
            for w in NodeIterator::new(Rc::clone(&min)) {
                let mut x = w;
                let mut d = x.get_degree();
                while let Some(mut y) = A[d].clone() {
                    if x.get_key() > y.get_key() {
                        // swap x y
                        let x_rc = Rc::clone(&x);
                        let y_rc = Rc::clone(&y);
                        x = y_rc;
                        y = x_rc;
                    }
                    self.link(Rc::clone(&y), Rc::clone(&x));
                    A[d] = None;
                    d += 1;

                    /* 動かないみたい
                    if d >= D {
                        println!("d={}>=D={}", &d, &D);
                        break;
                    }
                    */
                }
                x.remove();
                A[d] = Some(Rc::clone(&x));
            }
        }
        self.min = None;
        for i in 0..D + 2 {
            if let Some(a_i) = A[i].clone() {
                self.insert_node(Rc::clone(&a_i));

                if let Some(min) = self.get_min() {
                    if a_i.get_key() < min.get_key() {
                        self.min = Some(a_i);
                    }
                } else {
                    self.min = Some(a_i);
                }
            }
        }
    }

    fn link(&self, y: Rc<Node<T>>, x: Rc<Node<T>>) {
        y.remove();
        x.add_child(Rc::clone(&y));
        y.unmark();
    }

    pub fn decrease_key(&mut self, x: Rc<Node<T>>, k: isize) {
        if k > x.get_key() {
            panic!("新しいキーは現在のキーより大きい");
        }
        x.set_key(k);
        if let Some(y) = x.get_parent() {
            if x.get_key() < y.get_key() {
                self.cut(Rc::clone(&x), Rc::clone(&y));
                self.cascading_cut(Rc::clone(&y));
            }
        }
        if x.get_key() < self.get_min().unwrap().get_key() {
            self.min = Some(Rc::clone(&x));
        }
    }

    fn cut(&mut self, x: Rc<Node<T>>, y: Rc<Node<T>>) {
        x.remove();
        y.decrement_degree();
        self.insert_node(Rc::clone(&x));
        x.clear_parent();
        x.unmark();
    }

    fn cascading_cut(&mut self, y: Rc<Node<T>>) {
        if let Some(z) = y.get_parent() {
            if y.is_marked() == false {
                y.mark();
            } else {
                self.cut(Rc::clone(&y), Rc::clone(&z));
                self.cascading_cut(Rc::clone(&z));
            }
        }
    }

    pub fn delete(&mut self, x: Rc<Node<T>>) {
        self.decrease_key(x, isize::MIN);
        self.extract_min();
    }

    pub fn print(&self) {
        println!("n={}", self.n);
        if let Some(min) = self.get_min() {
            for node in NodeIterator::new(min) {
                node.print(0);
            }
        }
        println!("");
    }
}

pub struct Node<T> {
    key: RefCell<isize>,
    value: T,
    // 循環参照を避けるために一方向はRcポインタ、もう一方はWeakポインタを使用
    parent: RefCell<Option<Weak<Node<T>>>>,
    child: RefCell<Option<Rc<Node<T>>>>,
    right: RefCell<Option<Rc<Node<T>>>>,
    left: RefCell<Option<Weak<Node<T>>>>,
    degree: RefCell<usize>,
    is_marked: RefCell<bool>,
}

impl<T> Node<T> {
    pub fn construct_child(&self, input: Vec<InputData<T>>) {
        for x in input {
            let node = Node::new(x.key, x.value, x.is_marked);
            node.construct_child(x.children);
            self.add_child(node);
        }
    }

    pub fn new(key: isize, value: T, is_marked: bool) -> Rc<Self> {
        let node = Rc::new(Node {
            key: RefCell::new(key),
            value: value,
            parent: RefCell::new(None),
            child: RefCell::new(None),
            right: RefCell::new(None),
            left: RefCell::new(None),
            degree: RefCell::new(0),
            is_marked: RefCell::new(is_marked),
        });

        node.set_right(Rc::clone(&node));
        node.set_left(Rc::downgrade(&node));

        node
    }

    pub fn print(&self, depth: i32) {
        let mut s = String::new();
        if (depth >= 10) {
            panic!("depth={}", depth);
            return;
        }
        for _ in 0..depth {
            s.push_str("  ");
        }
        if self.is_marked() {
            println!("{}{}*({}):", s, self.get_key(), self.get_degree());
        } else {
            println!("{}{}({}):", s, self.get_key(), self.get_degree());
        }

        if let Some(child) = self.get_child() {
            for node in NodeIterator::new(child) {
                node.print(depth + 1);
            }
        }
    }

    fn is_marked(&self) -> bool {
        *self.is_marked.borrow()
    }

    fn mark(&self) {
        *self.is_marked.borrow_mut() = true;
    }

    fn unmark(&self) {
        *self.is_marked.borrow_mut() = false;
    }

    pub fn get_degree(&self) -> usize {
        *self.degree.borrow()
    }

    fn increment_degree(&self) {
        *self.degree.borrow_mut() += 1;
    }

    fn decrement_degree(&self) {
        *self.degree.borrow_mut() -= 1;
    }

    pub fn get_key(&self) -> isize {
        *self.key.borrow()
    }

    fn set_key(&self, key: isize) {
        *self.key.borrow_mut() = key;
    }

    fn set_right(&self, node: Rc<Self>) {
        *self.right.borrow_mut() = Some(node);
    }

    fn get_right(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_rc(self.right.borrow())
    }

    fn clear_right(&self) {
        *self.right.borrow_mut() = None;
    }

    fn set_left(&self, node: Weak<Self>) {
        *self.left.borrow_mut() = Some(node);
    }

    fn get_left(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_weak(self.left.borrow())
    }

    fn clear_left(&self) {
        *self.left.borrow_mut() = None;
    }

    fn set_parent(&self, node: Weak<Self>) {
        *self.parent.borrow_mut() = Some(node);
    }

    fn get_parent(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_weak(self.parent.borrow())
    }

    fn clear_parent(&self) {
        *self.parent.borrow_mut() = None;
    }

    fn set_child(&self, node: Rc<Self>) {
        *self.child.borrow_mut() = Some(node);
    }

    fn get_child(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_rc(self.child.borrow())
    }

    fn clear_child(&self) {
        *self.child.borrow_mut() = None;
    }

    fn from_ref_option_rc(node: Ref<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        if let Some(ref n) = *node {
            Some(Rc::clone(n))
        } else {
            None
        }
    }

    fn from_borrowed_option_rc(node: &Option<Rc<Self>>) -> Option<Rc<Self>> {
        if let &Some(ref n) = node {
            Some(Rc::clone(n))
        } else {
            None
        }
    }

    fn from_ref_option_weak(node: Ref<Option<Weak<Self>>>) -> Option<Rc<Self>> {
        if let Some(ref n) = *node {
            Weak::upgrade(n)
        } else {
            None
        }
    }

    fn remove(&self) {
        let left = self.get_left().unwrap();
        let right = self.get_right().unwrap();
        let node = self.get_self_rc();

        if !Rc::ptr_eq(&node, &right) {
            left.set_right(Rc::clone(&right));
            right.set_left(Rc::downgrade(&left));
        }

        if let Some(parent) = node.get_parent() {
            if let Some(child) = parent.get_child() {
                if Rc::ptr_eq(&node, &child) {
                    if !Rc::ptr_eq(&node, &right) {
                        parent.set_child(Rc::clone(&right));
                    } else {
                        parent.clear_child();
                    }
                }
            }
        }
        node.clear_parent();
        node.set_right(Rc::clone(&node));
        node.set_left(Rc::downgrade(&node));
    }

    fn extract(&self) -> Rc<Self> {
        let self_rc = self.get_self_rc();
        self.remove();
        self.set_right(Rc::clone(&self_rc));
        self.set_left(Rc::downgrade(&self_rc));
        self.clear_parent();
        self_rc
    }

    fn get_self_rc(&self) -> Rc<Self> {
        self.get_left().unwrap().get_right().unwrap()
    }

    fn concatenate(&self, node: Rc<Self>) {
        for sibling in NodeIterator::new(Rc::clone(&node)) {
            if let Some(parent) = self.get_parent() {
                sibling.set_parent(Rc::downgrade(&parent));
            } else {
                sibling.clear_parent();
            }
        }

        let self_rc = self.get_self_rc();
        let node_left = node.get_left().unwrap();
        let self_left = self.get_left().unwrap();

        node_left.set_right(Rc::clone(&self_rc));
        self.set_left(Rc::downgrade(&node_left));

        node.set_left(Rc::downgrade(&self_left));
        self_left.set_right(Rc::clone(&node));
    }

    pub fn add_child(&self, node: Rc<Self>) -> Rc<Self> {
        if let Some(child) = self.get_child() {
            child.concatenate(Rc::clone(&node));
        } else {
            self.set_child(Rc::clone(&node));
        }
        let self_rc = self.get_self_rc();
        node.set_parent(Rc::downgrade(&self_rc));
        self.increment_degree();
        Rc::clone(&self_rc)
    }
}

pub struct NodeIterator<T> {
    first: Rc<Node<T>>,
    current: Option<Rc<Node<T>>>,
    first_seen: bool,
    last_seen: bool,
}

impl<T> NodeIterator<T> {
    fn new(node: Rc<Node<T>>) -> Self {
        NodeIterator {
            first: Rc::clone(&node),
            current: Some(Rc::clone(&node)),
            first_seen: false,
            last_seen: false,
        }
    }
}

impl<T> Iterator for NodeIterator<T> {
    type Item = Rc<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = Node::from_borrowed_option_rc(&self.current)?;

        if self.first_seen && Rc::ptr_eq(&current, &self.first) {
            return None;
        } else if Rc::ptr_eq(&current, &self.first) {
            self.first_seen = true;
        } else if Rc::ptr_eq(&current, &current.get_right().unwrap()) {
            // イテレータを回しながら兄弟を削除するとfirst_seenが消されて無限ループに陥ることがある
            // 根リスト、子リストに一つのノードしか存在しない時、一回だけ返して終了させる
            if self.last_seen {
                return None;
            } else {
                self.last_seen = true;
            }
        }

        self.current = current.get_right();

        Some(current)
    }
}

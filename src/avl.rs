use std::rc::{Weak, Rc};
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct Node<T: Clone> {
    data: T,
    height: i32,
    counter: usize,
    left: Option<NodeRef<T>>,
    right: Option<NodeRef<T>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

enum NodeSide {
    Left,
    Right
}

impl<T: Clone> Node<T> {
    fn new(data: T, parent: Option<Weak<RefCell<Node<T>>>>) -> Node<T> {
        Self {  
            data,
            left: None,
            right: None,
            parent,
            height: 0,
            counter: 1,
        }
    }

    fn adjust_height(&mut self) {
        let left_height = match &self.left {
            Some(v) => v.borrow().height + 1,
            None => 0,
        };
        let right_height = match &self.right {
            Some(v) => v.borrow().height + 1,
            None => 0,
        };

        self.height = left_height.max(right_height);
    }

    fn get_balance_factor(&self) -> i32 {
        let left_height = match &self.left {
            Some(v) => v.borrow().height + 1,
            None => 0,
        };
        let right_height = match &self.right {
            Some(v) => v.borrow().height + 1,
            None => 0,
        };

        left_height - right_height
    }

    fn count_children(&self) -> usize {
        let mut ans = 0;
        if self.left.is_some() {
            ans += 1;
        }
        if self.right.is_some() {
            ans += 1;
        }

        ans
    }
}

pub struct MultiAVL<T>
    where T: Ord + Clone
{
    root: Option<NodeRef<T>>,
    size: usize,
    min_node: Option<NodeRef<T>>,
    max_node: Option<NodeRef<T>>,
}

impl<T: Ord + Clone> MultiAVL<T> {
    pub fn new() -> MultiAVL<T> {
        Self { 
            root: None, 
            size: 0,
            min_node: None,
            max_node: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, value: T) {
        let mut parent = None;
        let mut node = self.root.as_ref().map(Rc::clone);
        let mut side = NodeSide::Left;

        let mut is_max = true;
        let mut is_min = true;

        while let Some(n) = node {
            parent = Some( Rc::clone(&n) );
            if value == n.borrow().data {
                n.borrow_mut().counter += 1;
                self.size += 1;
                return;
            } else if value < n.borrow().data {
                is_max = false;
                side = NodeSide::Left;
                node = n.borrow().left.as_ref().map(Rc::clone);
            } else {
                is_min = false;
                side = NodeSide::Right;
                node = n.borrow().right.as_ref().map(Rc::clone);
            }
        }
        self.size += 1;

        let new_node = Rc::new(RefCell::new( Node::new( value, None )));

        if is_max {
            self.max_node = Some( Rc::clone(&new_node) );
        }
        if is_min {
            self.min_node = Some( Rc::clone(&new_node) );
        }

        if let Some(v) = &parent {
            match side {
                NodeSide::Left => Self::link_left_node(&v, &new_node),
                NodeSide::Right => Self::link_right_node(&v, &new_node),
            }
            self.rebalance( Rc::clone(v) );
        } else {
            self.root = Some( new_node );
        }
        
    }

    pub fn iter(&self) -> MultiAVLTreeIter<T>{
        if let Some(v) = self.min_iter() {
            return v;
        }
        MultiAVLTreeIter { now: None, counter: 0 }
    }

    pub fn max_value(&self) -> Option<T> {
        if let Some(v) = &self.max_node {
            let v = Rc::clone(v);
            return Some( v.borrow().data.clone() );
        }
        None
    }
    
    pub fn max_iter(&self) -> Option<MultiAVLTreeIter<T>> {
        if let Some(v) = &self.max_node {
            let v = Rc::clone(v);
            return Some( Self::node_to_iter(&v) );
        }
        None
    }
    
    pub fn min_value(&self) -> Option<T> {
        if let Some(v) = &self.min_node {
            let v = Rc::clone(v);
            return Some( v.borrow().data.clone() );
        }
        None
    }
    
    pub fn min_iter(&self) -> Option<MultiAVLTreeIter<T>> {
        if let Some(v) = &self.min_node {
            let v = Rc::clone(v);
            return Some( Self::node_to_iter(&v) );
        }
        None
    }

    pub fn contains(&self, value: T) -> bool {
        let node = self.find_node(&value);
        node.is_some()
    }

    pub fn erase(&mut self, value: T) {
        let node = self.find_node(&value);
        if node.is_none() {
            return;
        }
        self.erase_node(&node.unwrap());
    }

    pub fn erase_iter(&mut self, iter: MultiAVLTreeIter<T>) {
        if let Some(node) = iter.now {
            let node = Weak::upgrade(&node);
            if node.is_none() { return; }
            let node = node.unwrap();
            self.erase_node(&node)
        }
    }

    fn find_node(&self, value: &T) -> Option<NodeRef<T>> {
        let mut node = self.root.as_ref().map(Rc::clone);
        while let Some(n) = node.clone() {
            let n_borrow = n.borrow();
            if *value == n_borrow.data {
                break;
            } else if *value < n_borrow.data {
                node = n_borrow.left.as_ref().map(Rc::clone);
            } else {
                node = n_borrow.right.as_ref().map(Rc::clone);
            }
        }
        
        node
    }

    fn find_max_node(&self) -> Option<NodeRef<T>> {
        if self.root.is_none() {
            return None;
        }
        let mut node = self.root.as_ref().map(Rc::clone).unwrap();
        while let Some(n) = &node.clone().borrow().right {
            node = Rc::clone(&n);
        }
        Some(node)
    }

    fn find_min_node(&self) -> Option<NodeRef<T>> {
        if self.root.is_none() {
            return None;
        }
        let mut node = self.root.as_ref().map(Rc::clone).unwrap();
        while let Some(n) = &node.clone().borrow().left {
            node = Rc::clone(&n);
        }
        Some(node)
    }

    fn node_to_iter(node: &NodeRef<T>) -> MultiAVLTreeIter<T> {
        MultiAVLTreeIter { now: Some( Rc::downgrade(&node) ), counter: 1 }
    }

    fn remove_node(side: NodeSide, node: &NodeRef<T>) -> Option<NodeRef<T>> {
        let retu = match side {
            NodeSide::Left  => node.borrow_mut().left.take(),
            NodeSide::Right => node.borrow_mut().right.take()
        };
        Self::adjust_height(&node);
        retu
    }

    fn remove_left(node: &NodeRef<T>) -> Option<NodeRef<T>> {
        Self::remove_node(NodeSide::Left, node)
    }

    fn remove_right(node: &NodeRef<T>) -> Option<NodeRef<T>> {
        Self::remove_node(NodeSide::Right, node)
    }

    fn link_node(side: NodeSide, parent: &NodeRef<T>, child: &NodeRef<T>) {
        child.borrow_mut().parent = Some( Rc::downgrade(&parent) );
        match side {
            NodeSide::Left  => parent.borrow_mut().left = Some( Rc::clone(child) ),
            NodeSide::Right => parent.borrow_mut().right = Some( Rc::clone(child) )
        }

        Self::adjust_height(&parent);
    }

    fn link_right_node(parent: &NodeRef<T>, child: &NodeRef<T>) {
        Self::link_node(NodeSide::Right, parent, child);
    }

    fn link_left_node(parent: &NodeRef<T>, child: &NodeRef<T>) {
        Self::link_node(NodeSide::Left, parent, child);
    }

    fn is_max_node(&self, node: &NodeRef<T>) -> bool {
        if let Some(v) = &self.max_node {
            return Rc::ptr_eq(v, node);
        }
        false
    }

    fn is_min_node(&self, node: &NodeRef<T>) -> bool {
        if let Some(v) = &self.min_node {
            return Rc::ptr_eq(v, node);
        }
        false
    }

    fn erase_node(&mut self, node: &NodeRef<T>) {
        node.borrow_mut().counter -= 1;
        if node.borrow().counter > 0 {
            self.size -= 1;
            return;
        }

        //　最大最小を計算しなおすべきかどうか
        let mut recalc_min = false;
        let mut recalc_max = false;
        if self.is_max_node(node) {
            recalc_max = true
        }
        if self.is_min_node(node) {
            recalc_min = true;
        }

        let num_child = node.borrow().count_children();
        match num_child {
            0 => self.erase_node_no_child(node),
            1 => self.erase_node_one_child(node),
            2 => { 
                node.borrow_mut().counter += 1;
                self.erase_node_two_children(node)
            },
            _ => panic!("Unexpected number of children"),
        }

        if recalc_max {
            self.max_node = self.find_max_node();
        }
        if recalc_min {
            self.min_node = self.find_min_node();
        }
    }

    // nodeが親のどちらについているかを返す 根ノードの場合Noneが返る
    fn get_node_position(node: &NodeRef<T>) -> Option<NodeSide> {
        if let Some(parent) = &node.borrow().parent {
            let parent = parent.upgrade().unwrap();

            if let Some(parent_left) = &parent.borrow().left {
                if Rc::ptr_eq(&node, parent_left) {
                    return Some( NodeSide::Left );
                }
            }
            debug_assert!(Rc::ptr_eq(&parent.borrow().right.as_ref().unwrap(), node));
            return Some( NodeSide::Right );
        } else {
            None
        }
    }

    // 子を持たないノードの削除
    fn erase_node_no_child(&mut self, target: &NodeRef<T>) {
        debug_assert_eq!(target.borrow().count_children(), 0);
        if let Some(parent) = &target.borrow().parent {
            let parent = parent.upgrade().unwrap();

            // 親のどちらにいるかを取得
            let side = Self::get_node_position(&target).unwrap();
            match side {
                NodeSide::Left  => { Self::remove_left(&parent); },
                NodeSide::Right => { Self::remove_right(&parent); }
            }

            self.rebalance(parent);
        } else {
            self.root = None;
        }
        self.size -= 1;
    }

    // 子を１つだけ持つノードの削除
    fn erase_node_one_child(&mut self, target: &NodeRef<T>) {
        debug_assert_eq!(target.borrow().count_children(), 1);

        let child;
        if target.borrow().left.is_some() {
            child = Self::remove_left(&target).unwrap();
        } else {
            child = Self::remove_right(&target).unwrap();
        }

        if let Some(parent) = &target.borrow().parent {
            let parent = Weak::upgrade(parent).unwrap();

            let side: NodeSide = Self::get_node_position(target).unwrap();
            match side {
                NodeSide::Left => {
                    Self::link_left_node(&parent, &child);
                },
                NodeSide::Right => {
                    Self::link_right_node(&parent, &child);
                }
            }

            self.rebalance(parent);
        } else {
            child.borrow_mut().parent = None;
            self.root = Some( Rc::clone(&child) );
        };
        self.size -= 1;
    }

    // 子を２つもつノードの削除
    fn erase_node_two_children(&mut self, node: &NodeRef<T>) {
        debug_assert_eq!(node.borrow().count_children(), 2);

        let left_node = node.borrow().left.as_ref().map(Rc::clone).unwrap();
        // 左の部分木から最大ノードを探す
        let mut max_node = left_node;
        while let Some(v) = &Rc::clone(&max_node).borrow().right {
            max_node = Rc::clone(&v);
        }

        // 削除対象ノードと最大ノードのデータを入れ替える
        std::mem::swap(&mut node.borrow_mut().data, &mut max_node.borrow_mut().data);
        std::mem::swap(&mut node.borrow_mut().counter, &mut max_node.borrow_mut().counter);

        // 最大ノードだったノードを消す
        debug_assert!(max_node.borrow().right.is_none());
        self.erase_node(&max_node);
    }

    // ノードの高さを計算しなおす
    fn adjust_height(node: &NodeRef<T>) {
        let left_height = match &node.borrow().left {
            Some(v) => v.borrow().height + 1, 
            None => 0,
        };
        let right_height = match &node.borrow().right {
            Some(v) => v.borrow().height + 1,
            None => 0,
        };

        node.borrow_mut().height = left_height.max(right_height);
    }

    // nodeを根として左回転
    fn rotate_left(&mut self, node: &NodeRef<T>) {
        let right_child = Self::remove_right(&node);
        if right_child.is_none() {
            return;
        }
        let right_child = right_child.unwrap();
        
        // ノードの付け替え
        if let Some(left_node) = &Self::remove_left(&right_child) {
            Self::link_right_node(&node, &left_node);
        }

        match &node.borrow().parent {
            Some(v) => {
                let v = Weak::upgrade(&v).unwrap();
                if v.borrow().left.is_some() && Rc::ptr_eq(&node, v.borrow().left.as_ref().unwrap()) {
                    Self::link_left_node(&v, &right_child);
                } else {
                    Self::link_right_node(&v, &right_child);
                }
            },
            None => {
                self.root = Some( Rc::clone(&right_child) );
                right_child.borrow_mut().parent = None;
            }
        }

        Self::link_left_node(&right_child, &node);

        // 高さ調節
        Self::adjust_height(&node);
        Self::adjust_height(&right_child);
    }

    // nodeを根として右回転
    fn rotate_right(&mut self, node: &NodeRef<T>) {
        let left_child = Self::remove_left(&node);
        if left_child.is_none() {
            return;
        }
        let left_child = left_child.unwrap();

        //　ノードの付け替え
        if let Some(right_node) = &Self::remove_right(&left_child) {
            Self::link_left_node(&node, right_node);
        }

        match &node.borrow().parent {
            Some(v) => {
                let v = Weak::upgrade(v).unwrap();
                if v.borrow().left.is_some() && Rc::ptr_eq(&node, v.borrow().left.as_ref().unwrap()) {
                    Self::link_left_node(&v, &left_child);
                } else {
                    Self::link_right_node(&v, &left_child);
                }
            },
            None => {
                self.root = Some( Rc::clone(&left_child) );
                left_child.borrow_mut().parent = None;
            }
        }

        Self::link_right_node(&left_child, &node);

        //　高さ調整
        Self::adjust_height(&node);
        Self::adjust_height(&left_child);
    }

    // 二重回転が必要かどうか
    fn need_double_rot(node: &NodeRef<T>) -> bool {
        let n_balance = node.borrow().get_balance_factor();
        if n_balance == 2 {
            let mut c_balance = 0;
            if let Some(v) = &node.borrow().left {
                c_balance = v.borrow().get_balance_factor();
            }

            if c_balance == -1 {
                return true;
            }
        }
        if n_balance == -2 {
            let mut c_balance = 0;
            if let Some(v) = &node.borrow().right {
                c_balance = v.borrow().get_balance_factor();
            }

            if c_balance == 1 {
                return true;
            }
        }

        false
    }

    // nodeをリバランスする
    fn rebalance_node(&mut self, node: NodeRef<T>){
        node.borrow_mut().adjust_height();
        let balance = node.borrow().get_balance_factor();
        if balance == 2 {
            if Self::need_double_rot(&node) {
                let left_child = Rc::clone( &node.borrow().left.as_ref().unwrap() );
                self.rotate_left(&left_child);
            }
            self.rotate_right(&node);
        }else if balance == -2 {
            if Self::need_double_rot(&node) {
                let right_child = Rc::clone( &node.borrow().right.as_ref().unwrap() );
                self.rotate_right(&right_child);
            }
            self.rotate_left(&node);
        }
    }

    // nodeから上に根に向かってリバランスしていく
    fn rebalance(&mut self, node: NodeRef<T>) {
        let mut now = node;
        loop {
            let mut nxt = None;
            if let Some(v) = &now.borrow().parent {
                nxt = Some( Weak::upgrade(v).unwrap() );
            }
            Self::adjust_height(&now);
            self.rebalance_node(now);
            if let Some(v) = &nxt {
                now = Rc::clone(v);
            } else {
                break;
            }
        }
    }
}

pub struct MultiAVLTreeIter<T: Clone> {
    now: Option<Weak<RefCell<Node<T>>>>,
    counter: usize,
}

impl<T: Clone> Iterator for MultiAVLTreeIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.now.is_none(){ return None;}

        let node = Weak::upgrade(&self.now.as_ref().clone().unwrap());
        if node.is_none() {
            return  None;
        }

        
        let node = node.unwrap();
        self.counter += 1;
        if self.counter <= node.borrow().counter {
            return Some( node.borrow().data.clone() );
        } 
        self.counter = 1;

        let ret_data = node.borrow().data.clone();
        if let Some(v) = &node.clone().borrow().right {
            // 今のノードに右の子があるなら、右の子から可能な限り左に行く
            let mut now = Rc::clone(v);
            while let Some(nxt) = &now.clone().borrow().left {
                now = Rc::clone(nxt);    
            }

            self.now = Some( Rc::downgrade(&now) );
        } else {
            // 親の左の子になるまでたどる
            self.now = None;
            let mut now =  Rc::clone(&node);
            while let Some(parent) = &now.clone().borrow().parent {
                let parent = Weak::upgrade(parent);
                if parent.is_none() { return None; }
                let parent = parent.unwrap();

                // 左の子か確認 左の子であれば終わり
                if let Some(left) = &parent.clone().borrow().left {
                    if Rc::ptr_eq(left, &now) {
                        self.now = Some( Rc::downgrade(&parent) );
                        break;
                    }
                }

                now = parent;
            }
        }
        Some( ret_data )
    }
}

// テスト用関数
#[cfg(test)]
impl MultiAVL<i32> {
    pub fn check_consistent(&self) -> Result<(),Box<dyn std::error::Error>> {
        // ノードの親子関係をかくにんするものを作る
        self.is_size_correct()?;
        self.is_order_correct()?;
        self.is_relation_correct()?;
        self.is_node_height_correct()?;
        self.is_balanced()?;
        Ok(())
    }

    /*
        木の大小関係を確認する
        left < node < right
    */
    fn is_order_correct(&self) -> Result<(),Box<dyn std::error::Error>>{
        if let Some(v) = &self.root {
            Self::dfs_is_order_correct(v, None, None)?;
        }
        Ok(())
    }
    
    fn dfs_is_order_correct(node: &NodeRef<i32>, min_value: Option<i32>, max_value: Option<i32>) -> Result<(),Box<dyn std::error::Error>>{
        if let Some(max) = max_value {
            if node.borrow().data > max {
                return Err("order is not correct".into());
            }
        }

        if let Some(min) = min_value {
            if node.borrow().data < min {
                return Err("order is not correct".into());
            }
        }

        if let Some(left) = &node.borrow().left {
            Self::dfs_is_order_correct(left, min_value, Some( node.borrow().data ))?;
        }
        if let Some(right) = &node.borrow().right {
           Self::dfs_is_order_correct(right, Some( node.borrow().data ), max_value)?;
        }

        Ok(())
    }

    // 木の要素数を確認する
    fn is_size_correct(&self) -> Result<(),Box<dyn std::error::Error>> {
        let mut cnt = 0;
        if let Some(v) = &self.root {
            cnt = Self::dfs_size_correct(v)
        }
        if self.size == cnt {
            Ok(())
        } else {
            Err("size is not correct".into())
        }
    }

    fn dfs_size_correct(node: &NodeRef<i32>) -> usize {
        let mut cnt = node.borrow().counter;
        if let Some(v) = &node.borrow().left {
            cnt += Self::dfs_size_correct(v);
        }
        if let Some(v) = &node.borrow().right {
            cnt += Self::dfs_size_correct(v);
        }

        cnt
    }

    // 木の高さを確認する
    fn is_node_height_correct(&self) -> Result<(),Box<dyn std::error::Error>> {
        if let Some(v) = &self.root {
            Self::dfs_is_node_height_correct(v)?;
        }

        Ok(())
    }
    
    fn dfs_is_node_height_correct(node: &NodeRef<i32>) -> Result<i32, Box<dyn std::error::Error>> {
        let mut ans = 0;
        if let Some(v) = &node.borrow().left {
            let left_height = Self::dfs_is_node_height_correct(v)?;
            ans = ans.max(1 + left_height);
        }
        if let Some(v) = &node.borrow().right {
            let right_height = Self::dfs_is_node_height_correct(v)?;
            ans = ans.max(1 + right_height);
        }

        if node.borrow().height == ans {
            Ok(ans)
        } else {
            Err("node height is not correct".into())
        }
    }

    // ノードの親子関係を確認する
    fn is_relation_correct(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(v) = &self.root {
            Self::dfs_is_relation_correct(v)?;
            if v.borrow().parent.is_some() {
                return Err("relation is not correct".into());
            }
        }

        Ok(())
    }

    fn check_relation(node: &NodeRef<i32>, child: &NodeRef<i32>) -> bool {
        let child_parent = &child.borrow().parent;
        if child_parent.is_none() { return false; } // 親要素が設定されてるか
        let child_parent = child_parent.as_ref().unwrap();

        let child_parent = child_parent.upgrade();
        if child_parent.is_none() { return false; } // 親要素が生きてるか

        let child_parent = &child_parent.unwrap();
        Rc::ptr_eq(node, &child_parent) // 参照が正しいか 

    }

    fn dfs_is_relation_correct(node: &NodeRef<i32>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(left) = &node.borrow().left {
            if !Self::check_relation(node, left) {
                return Err("relation is not correct".into());
            }
        }
        if let Some(right) = &node.borrow().right {
            if !Self::check_relation(node, right) {
                return Err("relation is not correct".into());
            }
        }

        Ok(())
    }


    /*
        木のバランスを確認する
        -1 <= left.height - right.height <= 1
    */
    fn is_balanced(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(v) = &self.root {
            Self::dfs_is_balanced(v)?;
        }
        
        Ok(())
    }

    fn dfs_is_balanced(node: &NodeRef<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let balance = node.borrow().get_balance_factor();
        
        if balance < -1 || balance > 1 {
            return Err("tree is not balanced".into());
        }
        if let Some(v) = &node.borrow().left {
            Self::dfs_is_balanced(v)?
        }
        if let Some(v) = &node.borrow().right {
            Self::dfs_is_balanced(v)?
        }
        Ok(())   
    }
}

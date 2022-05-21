mod tests;
pub mod sheetdata;
use std::{cell::RefCell, fmt::Display, ops::Deref, rc::Rc};

pub enum TreeSide {
    Left,
    Right,
}

#[derive(Default)]
pub struct BinaryTree<T: Copy> {
    data: T,
    left: Option<Rc<RefCell<BinaryTree<T>>>>,
    right: Option<Rc<RefCell<BinaryTree<T>>>>,
}

impl<T: Copy> BinaryTree<T> {
    pub fn new(value: T) -> BinaryTree<T> {
        let temp = BinaryTree {
            data: value,
            left: Default::default(),
            right: Default::default(),
        };

        return temp;
    }

    pub fn get_value(&self) -> T {
        let value = self.data;
        return value;
    }
    pub fn set_value(&mut self, value: T) {
        self.data = value;
    }
    pub fn insert_tree(&mut self, side: TreeSide, binary_tree: Rc<RefCell<BinaryTree<T>>>) {
        match side {
            TreeSide::Left => self.left = Some(binary_tree),
            TreeSide::Right => self.right = Some(binary_tree),
        }
    }
    pub fn delete_tree(&mut self, side: TreeSide) {
        match side {
            TreeSide::Left => self.left = None,
            TreeSide::Right => self.right = None,
        }
    }
}

//遍历二叉树,统计每个分支最终的和sum;
pub fn traverse_branch_sum<
    T: Copy+ std::ops::Add<Output = T> + std::convert::Into<f64>,
>(
    binary_tree: Rc<RefCell<BinaryTree<T>>>,
    mut sum: T,
    sum_vec: &mut Vec<T>,
) {
    sum = sum + binary_tree.deref().borrow().data;
    //println!("{}",sum);
    //println!("{}",(*binary_tree).borrow().data);
    match &binary_tree.clone().deref().borrow().left {
        None => {
            sum_vec.push(sum);
        }
        Some(x) => {
            traverse_branch_sum(x.clone(), sum, sum_vec);
        }
    }
    match &binary_tree.clone().deref().borrow().right {
        None => {}
        Some(x) => {
            traverse_branch_sum(x.clone(), sum, sum_vec);
        }
    }
}

//对n个对象,每个对象都有两个可能值, 该函数将所有可能生成的分支,通过二叉树表示出来;该二叉数是等臂的;
//输入的两个同长度的矩阵,和二叉树头;
pub fn n2_permulation<T: Copy + std::ops::Add<Output = T> >(
    array1: &Vec<T>,
    array2: &Vec<T>,
    head_value: T,
) -> Rc<RefCell<BinaryTree<T>>> {
    if array1.len() != array2.len() {
        panic!("Two arrays' lengh is not equality!")
    }
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();

    for i in array1 {
        v1.push(Rc::new(RefCell::new(BinaryTree::new(*i))));
    }

    for i in array2 {
        v2.push(Rc::new(RefCell::new(BinaryTree::new(*i))));
    }
    let length = v2.len();
    v3.push(v1);
    v3.push(v2);
    for i in 0..length {
        let m = length - i - 1usize;
        if m == 0 {
            break;
        }
        let n = m - 1usize;

        (*v3[0][n])
            .borrow_mut()
            .insert_tree(TreeSide::Left, v3[0][m].clone());
        (*v3[0][n])
            .borrow_mut()
            .insert_tree(TreeSide::Right, v3[1][m].clone());

        (*v3[1][n])
            .borrow_mut()
            .insert_tree(TreeSide::Left, v3[0][m].clone());
        (*v3[1][n])
            .borrow_mut()
            .insert_tree(TreeSide::Right, v3[1][m].clone());
    }

    let binarytree_head = BinaryTree {
        data: head_value,
        left: Some(v3[0][0].clone()),
        right: Some(v3[1][0].clone()),
    };
    let rc_refcell_binarytree_head = Rc::new(RefCell::new(binarytree_head));

    return rc_refcell_binarytree_head;
}

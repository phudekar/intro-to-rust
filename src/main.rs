fn main() {
    let mut tree = BinaryTree::new(6);
    tree.push(4);
    tree.push(8);
    tree.push(2);
    tree.push(1);
    tree.push(10);
    tree.push(7);
    tree.push(9);
    tree.push(3);
    tree.push(5);

    println!("{:#?}", tree);

    println!("{}", tree.contains(&4));
    println!("{}", tree.contains(&11));
}

#[derive(Debug)]
struct BinaryTree<T>
where
    T: PartialOrd,
{
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T>
where
    T: PartialOrd,
{
    pub fn new(t: T) -> Self {
        BinaryTree {
            value: t,
            left: None,
            right: None,
        }
    }

    pub fn push(&mut self, t: T) {
        if t < self.value {
            if let Some(left) = self.left.as_mut() {
                left.push(t);
            } else {
                self.left = Some(Box::new(BinaryTree::new(t)));
            }
        } else {
            if let Some(right) = self.right.as_mut() {
                right.push(t);
            } else {
                self.right = Some(Box::new(BinaryTree::new(t)));
            }
        }
    }

    pub fn contains(&self, t: &T) -> bool {
        if t.eq(&self.value) {
            return true;
        }
        self.left.as_ref().map(|l| l.contains(t)).unwrap_or(false)
            || self.right.as_ref().map(|r| r.contains(t)).unwrap_or(false)
    }
}

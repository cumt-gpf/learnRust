use std::cmp::PartialEq;

pub trait NodeKey:  PartialOrd + PartialEq + Copy {}
impl<T: PartialOrd + PartialEq + Copy> NodeKey for T {}

#[derive(Debug)]
pub struct BST<T:NodeKey> {
	root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: NodeKey> {
	elem: T,
	left: Link<T>,
	right: Link<T>,
}

impl<T: NodeKey> BST<T> {
	// add code here
	pub fn new() -> Self {
		BST { root: None }
	}

	pub fn insert(&mut self, elem: T) -> bool {
		match self.root {
			Some(ref mut node) => {
				if node.search(elem) {
					false
				}
				else {
					node.insert(elem)
				}

			},
			None => {
				let new_node = Box::new(Node {
					elem: elem,
					left: None,
					right: None,
				});
				self.root = Some(new_node);
				true
			},
		}


	}
	pub fn search(&mut self, elem: T) -> bool {
		match self.root {
			None => false,
			Some(ref mut boxed_node) => {
				boxed_node.search(elem)
			},
		}

	}

}




impl<T: NodeKey> Node<T> {
	pub fn search(&mut self, elem: T) -> bool {
		if self.elem == elem {
			true
		}
		else if self.elem < elem {
			match self.right {
				None => false,
				Some(ref mut node) => {
					node.search(elem)
				},
			}
		}
		else {
			match self.left {
				None => false,
				Some(ref mut node) => {
					node.search(elem)
				},
			}
		}
	}

	pub fn insert(&mut self, elem: T) -> bool {
		if self.elem < elem {
			match self.right {
				None => {
					let new_node = Box::new(Node {
						elem: elem,
						left: None,
						right: None,	
					});
					self.right = Some(new_node);
					true
				},
				Some(ref mut node) => {
					node.insert(elem)
				},
			}
		}
		else {
			match self.left {
				None => {
					let new_node = Box::new(Node {
						elem: elem,
						left: None,
						right: None,	
					});
					self.left = Some(new_node);
					true
				},
				Some(ref mut node) => {
					node.insert(elem)
				},
			}

		}
	}

	/*
	fn drop(&mut self) {
		if self.right == None && self.left == None  {
			mem::replace(&mut self, None);
			return;
		}
		match self.left {
			None => {},
			Some(ref mut node) => { node.drop(); },
		}

		match self.right {
			None => {},
			Some(ref mut node) => { node.drop(); },
		}

			
	}
	*/
}
/*
impl Drop for BST {
	// add code here
	fn drop(&mut self) {
		match self.root {
			Some(ref mut node) => { node.drop() },
			None => {},
		}
	}
}
*/


#[cfg(test)]
mod tests {
	use super::BST;

	#[test]
	fn basics() {
		let mut bst = BST::new();

		assert_eq!(bst.search(5), false);

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		println!("{:?}", bst);

		assert_eq!(bst.search(5), true);
	}

	#[test]

	fn testT() {
		let mut bst = BST::new();

		bst.insert(1.1);
		bst.insert(1.2);
		bst.insert(1.3);

		assert_eq!(bst.search(1.2), true);
	}
}


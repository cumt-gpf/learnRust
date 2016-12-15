#[derive(Debug)]
pub struct BST {
	root: Link,
}

//type Link = Option<Box<Node>>;
#[derive(Debug)]
enum Link {
	Empty,
	More(Box<Node>),
}

#[derive(Debug)]
struct Node {
	elem: i32,
	left: Link,
	right: Link,
}

impl BST {
	// add code here
	pub fn new() -> Self {
		BST { root: Link::Empty }
	}

	pub fn insert(&mut self, elem: i32) -> bool {
		match self.root {
			Link::More(ref mut node) => {
				if node.search(elem) {
					false
				}
				else {
					node.insert(elem)
				}

			},
			Link::Empty => {
				let new_node = Box::new(Node {
					elem: elem,
					left: Link::Empty,
					right: Link::Empty,
				});
				self.root = Link::More(new_node);
				true
			},
		}


	}
	pub fn search(&self, elem: i32) -> bool {
		match self.root {
			Link::Empty => false,
			Link::More(ref boxed_node) => {
				boxed_node.search(elem)
			},
		}

	}

}

impl Node {
	pub fn search(&self, elem: i32) -> bool {
		if self.elem == elem {
			true
		}
		else if self.elem < elem {
			match self.right {
				Link::Empty => false,
				Link::More(ref node) => {
					node.search(elem)
				},
			}
		}
		else {
			match self.left {
				Link::Empty => false,
				Link::More(ref node) => {
					node.search(elem)
				},
			}
		}
	}

	pub fn insert(&mut self, elem: i32) -> bool {
		if self.elem < elem {
			match self.right {
				Link::Empty => {
					let new_node = Box::new(Node {
						elem: elem,
						left: Link::Empty,
						right: Link::Empty,	
					});
					self.right = Link::More(new_node);
					true
				},
				Link::More(ref mut node) => {
					node.insert(elem)
				},
			}
		}
		else {
			match self.left {
				Link::Empty => {
					let new_node = Box::new(Node {
						elem: elem,
						left: Link::Empty,
						right: Link::Empty,	
					});
					self.left = Link::More(new_node);
					true
				},
				Link::More(ref mut node) => {
					node.insert(elem)
				},
			}

		}
	}
}

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

		assert_eq!(bst.search(3), true);
	}
}


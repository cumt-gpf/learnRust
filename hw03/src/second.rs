use std::cmp::PartialEq;

pub trait NodeKey:  PartialOrd + PartialEq + Copy {}
impl<T: PartialOrd + PartialEq + Copy> NodeKey for T {}

#[derive(Debug)]
pub struct Bst<T:NodeKey> {
	root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: NodeKey> {
	elem: T,
	left: Link<T>,
	right: Link<T>,
}

pub struct IntoIter<T: NodeKey>(Bst<T>);

pub struct Iter<'a, T: 'a + NodeKey > {
	next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T:'a + NodeKey> {
	next: Option<&'a mut Node<T>>,
}

impl<T: NodeKey> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.root.take().map(|node| {
			let node = *node;
			self.0.root = node.right;
			node.elem
		})
	}
}

impl<'a, T: NodeKey> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.right.as_ref().map(|node| &**node);
			&node.elem
		})
	}

}

impl<'a, T: NodeKey> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.right.as_mut().map(|node| &mut **node);
			&mut node.elem
		})
	}

}

impl<T: NodeKey> IntoIterator for Bst<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter {
		self.into_iter()	
	}
}

impl<'a, T:NodeKey> IntoIterator for &'a Bst<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}


impl <'a, T:NodeKey> IntoIterator for &'a mut Bst<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;
	fn into_iter(mut self) -> Self::IntoIter {
		self.iter_mut()
	}

}



impl<T: NodeKey> Bst<T> {
	// add code here
	pub fn new() -> Self {
		Bst { root: None }
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

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}

	pub fn iter(&self) -> Iter<T> {
		Iter {
			next: self.root.as_ref().map(|node| &**node)
		}
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut {
			next: self.root.as_mut().map(|node| &mut **node)
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
impl Drop for Bst {
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
	use super::Bst;

	#[test]
	fn basics() {
		let mut bst = Bst::new();

		assert_eq!(bst.search(5), false);

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		println!("{:?}", bst);

		assert_eq!(bst.search(5), true);
	}

	#[test]

	fn gen_test() {
		let mut bst = Bst::new();

		bst.insert(1.1);
		bst.insert(1.2);
		bst.insert(1.3);

		assert_eq!(bst.search(1.2), true);
	}

	#[test]
	fn into_iter() {
		let mut bst = Bst::new();

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		bst.insert(9);
		bst.insert(11);

		let mut iter = bst.into_iter();

		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), Some(7));
		assert_eq!(iter.next(), Some(9));

	}

	#[test]
	fn intoIterator() {
		let mut bst = Bst::new();

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		bst.insert(9);
		bst.insert(11);

		for elt in bst {
			println!("{}", elt);
		}

	}

	#[test]
	fn iter() {

		let mut bst = Bst::new();

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		bst.insert(9);
		bst.insert(11);

		let mut iter = bst.iter();

		assert_eq!(iter.next(), Some(&5));
		assert_eq!(iter.next(), Some(&7));
		assert_eq!(iter.next(), Some(&9));

	}

	#[test]
	fn intoIterator_ref() {
		let mut bst = Bst::new();

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		bst.insert(9);
		bst.insert(11);


		for elt in &bst {
			println!("{}", elt);
		}
	}

	#[test]
	fn iter_mut() {
		let mut bst = Bst::new();

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		bst.insert(9);
		bst.insert(11);

		let mut iter = bst.iter_mut();

		assert_eq!(iter.next(), Some(&mut 5));
		assert_eq!(iter.next(), Some(&mut 7));
		assert_eq!(iter.next(), Some(&mut 9));

		
	}


	#[test]
	fn iter_mut_2() {
		let mut bst = Bst::new();

		bst.insert(5);
		bst.insert(3);
		bst.insert(7);
		bst.insert(9);
		bst.insert(11);

		for elt in &mut bst {
			println!("{}", elt);
		}
		

		
	}

}


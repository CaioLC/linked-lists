use std::ptr;

pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>
}

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: ptr::null_mut(), tail: ptr::null_mut()}
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            });
            let raw_tail = Box::into_raw(new_tail);
            // .is_null checks for null, equivalent to checking for None
            if !self.tail.is_null() {
                // if the old tail existed, update it to point to the new tail
                    (*self.tail).next = raw_tail;
            } else {
                // otherwise, update the head to point to it
                self.head = raw_tail
            }
            self.tail = raw_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None
            }
            let old_head = self.head;
            self.head = (*old_head).next;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            let node = Box::from_raw(old_head);
            Some(node.elem)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
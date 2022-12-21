use std::cell::RefCell;
use std::rc::Rc;

use crate::{utils::extract_integers, Solution};

#[derive(Debug, Clone)]
struct Item {
    val: isize,
    pos: usize,
}

struct File {
    order: Vec<Rc<RefCell<Item>>>,
    items: Vec<Rc<RefCell<Item>>>,
}

impl File {
    fn from_items(items: &[isize], key: isize) -> Self {
        let order: Vec<_> = items
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                Rc::new(RefCell::new(Item {
                    val: v * key,
                    pos: i,
                }))
            })
            .collect();

        File {
            order: order.clone(),
            items: order,
        }
    }

    fn move_item(&mut self, item: usize) {
        let pos = self.order[item].borrow().pos;
        let dist = self.order[item].borrow().val;
        let new = (pos as isize + dist).rem_euclid(self.order.len() as isize - 1) as usize;

        if new > pos {
            self.items[pos].borrow_mut().pos = new;
            self.items[pos + 1..=new]
                .iter_mut()
                .for_each(|e| e.borrow_mut().pos -= 1);
            self.items[pos..=new].rotate_left(1);
        } else if new < pos {
            self.items[pos].borrow_mut().pos = new;
            self.items[new..=pos - 1]
                .iter_mut()
                .for_each(|e| e.borrow_mut().pos += 1);
            self.items[new..=pos].rotate_right(1);
        }
    }

    fn mix(mut self, count: usize) -> Self {
        for _ in 0..count {
            for i in 0..self.order.len() {
                self.move_item(i)
            }
        }

        self
    }

    fn coords(&self) -> isize {
        let zero = self.order.iter().find(|&p| p.borrow().val == 0).unwrap();
        let pos = zero.borrow().pos;

        [1000, 2000, 3000]
            .iter()
            .map(|off| self.items[(pos + off) % self.order.len()].borrow().val)
            .sum()
    }
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let input: Vec<isize> = extract_integers(input);

    let part1 = File::from_items(&input, 1).mix(1).coords();
    let part2 = File::from_items(&input, 811589153).mix(10).coords();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("1\n2\n-3\n3\n-2\n0\n4") == crate::Solution(3, 1623178306));
    }
}

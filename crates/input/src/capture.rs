//! Picking a new binding in the Keybinds tab (after WitherBlitz's
//! NSMB-Wii-Recompilation, `keybinds.cpp`): what is pressed after the
//! capture begins, up to `max` keys or buttons held together, taken once
//! they are all let go. What was already held when it began (the press that
//! started it) only counts once let go and pressed again.

pub struct Capture<C> {
    before: Vec<C>,
    picked: Vec<C>,
    max: usize,
}

impl<C: Copy + PartialEq> Capture<C> {
    /// A capture beginning with `held` held.
    pub fn new(held: &[C], max: usize) -> Self {
        Self { before: held.to_vec(), picked: Vec::new(), max: max.max(1) }
    }

    /// What has been picked so far, in the order pressed.
    pub fn picked(&self) -> &[C] {
        &self.picked
    }

    /// `held`: everything held now. Returns the binding once something was
    /// picked and all of it has been let go.
    pub fn step(&mut self, held: &[C]) -> Option<Vec<C>> {
        self.before.retain(|c| held.contains(c));
        for &c in held {
            if !self.before.contains(&c) && !self.picked.contains(&c) && self.picked.len() < self.max {
                self.picked.push(c);
            }
        }
        (!self.picked.is_empty() && !self.picked.iter().any(|c| held.contains(c))).then(|| std::mem::take(&mut self.picked))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn held_at_the_start_counts_once_let_go() {
        let mut c = Capture::new(&[1], 3);
        assert_eq!(c.step(&[1]), None);
        assert!(c.picked().is_empty());
        assert_eq!(c.step(&[]), None);
        assert_eq!(c.step(&[1]), None);
        assert_eq!(c.picked(), &[1]);
        assert_eq!(c.step(&[]), Some(vec![1]));
    }

    #[test]
    fn keys_together_are_taken_once_all_are_let_go() {
        let mut c = Capture::new(&[], 3);
        assert_eq!(c.step(&[5]), None);
        assert_eq!(c.step(&[5, 6]), None);
        assert_eq!(c.step(&[6]), None, "one still held");
        assert_eq!(c.step(&[6, 7, 8]), None);
        assert_eq!(c.picked(), &[5, 6, 7], "three at most");
        assert_eq!(c.step(&[8]), Some(vec![5, 6, 7]));
        let mut one = Capture::new(&[], 1);
        assert_eq!(one.step(&[2, 3]), None);
        assert_eq!(one.step(&[3]), Some(vec![2]));
    }
}

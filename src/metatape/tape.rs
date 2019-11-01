use std::fmt;
use std::fmt::Write;
use std::sync::Arc;

#[derive(Clone)]
pub struct Head {
    parent: Option<Arc<Tape>>, // extends up
    child: Option<Arc<Tape>>,  // extends down
    left: Option<Arc<Cell>>,   // extends left
    right: Option<Arc<Cell>>,  // extends right
}

struct Tape {
    next: Option<Arc<Tape>>,  // extends up/down
    left: Option<Arc<Cell>>,  // extends left
    right: Option<Arc<Cell>>, // extends right
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(left) = &self.left {
            let left_neighbors = if f.alternate() {
                format!("{:#}", left)
            } else {
                format!("{}", left)
            };
            f.write_str(&left_neighbors.chars().rev().collect::<String>())?;
        }
        f.write_char('[')?;
        fmt_cell_depth(f, &self.child)?;
        f.write_char(']')?;
        if let Some(right) = &self.right {
            if f.alternate() {
                f.write_str(&format!("{:#}", right))?;
            } else {
                f.write_str(&format!("{}", right))?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(' ')?;
        fmt_cell_depth(f, &self.child)?;
        f.write_char(' ')?;
        if let Some(next) = &self.next {
            if f.alternate() {
                f.write_str(&format!("{:#}", next))?;
            } else {
                f.write_str(&format!("{}", next))?;
            }
        }
        Ok(())
    }
}

fn fmt_cell_depth(f: &mut fmt::Formatter<'_>, child: &Option<Arc<Tape>>) -> fmt::Result {
    f.write_char(match child {
        None => '_',
        Some(child) => {
            if f.alternate() {
                let depth = child.get_depth();
                if depth > 9 {
                    '#'
                } else {
                    format!("{}", depth).chars().next().unwrap()
                }
            } else {
                '#'
            }
        }
    })
}

impl std::default::Default for Tape {
    fn default() -> Self {
        Self {
            next: None,
            left: None,
            right: None,
        }
    }
}

struct Cell {
    child: Option<Arc<Tape>>, // extends down
    next: Option<Arc<Cell>>,  // extends left/right
}

impl std::default::Default for Cell {
    fn default() -> Self {
        Self {
            child: None,
            next: None,
        }
    }
}

impl Head {
    pub fn new() -> Head {
        Head {
            left: None,
            right: None,
            parent: None,
            child: None,
        }
    }

    pub fn move_left(&self) -> Head {
        let left = self.left.clone().unwrap_or_default();
        Head {
            parent: self.parent.clone(),
            child: left.child.clone(),
            left: left.next.clone(),
            right: if let (None, None) = (&self.right, &self.child) {
                None
            } else {
                Some(Arc::new(Cell {
                    child: self.child.clone(),
                    next: self.right.clone(),
                }))
            },
        }
    }

    pub fn move_right(&self) -> Head {
        let right = self.right.clone().unwrap_or_default();
        Head {
            parent: self.parent.clone(),
            child: right.child.clone(),
            left: if let (None, None) = (&self.left, &self.child) {
                None
            } else {
                Some(Arc::new(Cell {
                    child: self.child.clone(),
                    next: self.left.clone(),
                }))
            },
            right: right.next.clone(),
        }
    }

    pub fn enter(&self) -> Head {
        let child = self.child.clone().unwrap_or_default();
        Head {
            parent: if let (None, None, None) = (&self.left, &self.parent, &self.right) {
                None
            } else {
                Some(Arc::new(Tape {
                    next: self.parent.clone(),
                    left: self.left.clone(),
                    right: self.right.clone(),
                }))
            },
            child: child.next.clone(),
            left: child.left.clone(),
            right: child.right.clone(),
        }
    }

    pub fn exit(&self) -> Head {
        let parent = self.parent.clone().unwrap_or_default();
        Head {
            parent: parent.next.clone(),
            child: Some(Arc::new(Tape {
                next: self.child.clone(),
                left: self.left.clone(),
                right: self.right.clone(),
            })),
            left: parent.left.clone(),
            right: parent.right.clone(),
        }
    }

    fn set_child(&self, new_child: Option<Arc<Tape>>) -> Head {
        Head {
            parent: self.parent.clone(),
            child: new_child,
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }

    pub fn null_child(&self) -> Head {
        self.set_child(None)
    }

    pub fn has_child(&self) -> bool {
        self.child.is_some()
    }

    pub fn copy_child_from(&self, other: &Head) -> Head {
        self.set_child(other.child.clone())
    }
}

impl Tape {
    fn get_depth(&self) -> usize {
        match &self.next {
            None => 0,
            Some(next) => next.get_depth() + 1,
        }
    }
}

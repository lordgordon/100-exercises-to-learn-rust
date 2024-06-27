pub struct DropBomb {
    active: bool
}

impl DropBomb {
    pub fn new() -> Self {
        Self {
            active: true,
        }
    }

    pub fn defuse(mut self) {
        self.active = false
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.active {
            panic!("Explosion")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}

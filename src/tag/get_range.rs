use super::Tag;

pub fn get_range(tags: Vec<Tag>, prefix: Tag) -> Option<std::ops::Range<usize>> {
    let prefixes: Vec<Tag> = {
        let mut prefixes = Vec::new();
        let last_index = prefix.get_last_index().unwrap();
        for i in (0..=last_index).rev() {
            let prefix = prefix.mutate_last_index(|_| i);
            prefixes.push(prefix);
        }
        prefixes
    };

    let start: usize = tags
        .iter()
        .enumerate()
        .find(|(_, t)| prefixes.iter().any(|p| p.is_prefix(t)))
        .map(|(i, _)| i)?;

    let end: usize = tags
        .iter()
        .enumerate()
        .find(|(i, t)| start < *i && !prefixes.iter().any(|p| p.is_prefix(t)))
        .map(|(i, _)| i)?;

    Some(start..end)
}

impl Tag {
    fn is_prefix(&self, other: &Self) -> bool {
        let self_len = self.0.len();
        let other_len = other.0.len();
        self_len <= other_len && self.0 == &other.0[..self_len]
    }

    fn get_last_index(&self) -> Option<usize> {
        self.0.last().copied()
    }

    fn mutate_last_index<F>(&self, f: F) -> Self
    where
        F: Fn(usize) -> usize,
    {
        let mut indices = self.0.clone();
        match indices.pop() {
            Some(last_index) => {
                let last_index = f(last_index);
                indices.push(last_index);
                Tag(indices)
            }
            None => Tag::empty(),
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prefix() {
        let tag: Tag = vec![3, 2, 0].into();

        let prefix: Tag = vec![3, 2, 0].into();
        assert!(prefix.is_prefix(&tag));

        let prefix: Tag = vec![3, 2].into();
        assert!(prefix.is_prefix(&tag));

        let prefix: Tag = vec![3].into();
        assert!(prefix.is_prefix(&tag));

        let prefix: Tag = vec![3, 1].into();
        assert!(!prefix.is_prefix(&tag));

        let prefix: Tag = vec![3, 0].into();
        assert!(!prefix.is_prefix(&tag));
    }
}

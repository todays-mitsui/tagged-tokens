use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct Tag(Vec<usize>);

impl Tag {
    pub fn empty() -> Self {
        Tag(Vec::new())
    }

    pub fn new(index: usize) -> Self {
        Tag(vec![index])
    }

    pub fn push(&self, index: usize) -> Self {
        let mut indices = self.0.clone();
        indices.push(index);
        Tag(indices)
    }

    pub fn get_last(&self) -> Option<usize> {
        self.0.last().map(|i| i.clone())
    }

    pub fn mutate_last<F>(&self, f: F) -> Self
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

impl From<Vec<usize>> for Tag {
    fn from(indices: Vec<usize>) -> Self {
        Tag(indices)
    }
}

impl Tag {
    pub fn prefix(&self, other: &Self) -> bool {
        let self_len = self.0.len();
        let other_len = other.0.len();
        self_len <= other_len && self.0 == &other.0[..self_len]
    }
}

// ========================================================================== //

pub fn get_range(tags: Vec<Tag>, tag: Tag) -> Option<Range<usize>> {
    let prefixs: Vec<Tag> = {
        let mut prefixs = Vec::new();
        let last_index = tag.get_last().unwrap();
        for i in (0..=last_index).rev() {
            let prefix = tag.mutate_last(|_| i);
            prefixs.push(prefix);
        }
        prefixs
    };

    let maybe_start: Option<usize> = tags
        .iter()
        .enumerate()
        .find(|(_, t)| prefixs.iter().any(|p| p.prefix(t)))
        .map(|(i, _)| i);

    if let None = maybe_start {
        return None;
    }

    let start = maybe_start.unwrap();

    let maybe_end: Option<usize> = tags
        .iter()
        .enumerate()
        .find(|(i, t)| start < *i && !prefixs.iter().any(|p| p.prefix(t)))
        .map(|(i, _)| i);

    let end = maybe_end.unwrap_or(tags.len());

    return Some(start..end);
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(Tag::empty(), Tag(vec![]));
    }

    #[test]
    fn test_new() {
        assert_eq!(Tag::new(0), Tag(vec![0]));
    }

    #[test]
    fn test_push() {
        assert_eq!(Tag::empty().push(0), Tag(vec![0]));
        assert_eq!(Tag::empty().push(0).push(1), Tag(vec![0, 1]));
    }

    #[test]
    fn test_prefix() {
        let tag: Tag = vec![3, 2, 0].into();

        let prefix: Tag = vec![3, 2, 0].into();
        assert!(prefix.prefix(&tag));

        let prefix: Tag = vec![3, 2].into();
        assert!(prefix.prefix(&tag));

        let prefix: Tag = vec![3].into();
        assert!(prefix.prefix(&tag));

        let prefix: Tag = vec![3, 1].into();
        assert!(!prefix.prefix(&tag));

        let prefix: Tag = vec![3, 0].into();
        assert!(!prefix.prefix(&tag));
    }
}

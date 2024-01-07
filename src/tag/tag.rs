use super::Tag;

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
}

impl From<Vec<usize>> for Tag {
    fn from(indices: Vec<usize>) -> Self {
        Tag(indices)
    }
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
}

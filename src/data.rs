use crate::prelude::*;
use rand::prelude::SliceRandom;

/// One hot encodes an array of class labels into a [Tensor2D] of probability
/// vectors. This can be used in tandem with [cross_entropy_with_logits_loss()].
///
/// Const Generic Arguments:
/// - `B` - the batch size
/// - `N` - the number of classes
///
/// Arguments:
/// - `class_labels` - an array of size `B` where each element is the class label
///
/// Outputs: [Tensor2D] with shape (B, N)
///
/// Examples:
/// ```rust
/// # use dfdx::prelude::*;
///
/// let class_labels = [0, 1, 2, 1, 1];
/// // NOTE: 5 is the batch size, 3 is the number of classes
/// let probs = one_hot_encode::<5, 3>(&class_labels);
/// assert_eq!(probs.data(), &[
///     [1.0, 0.0, 0.0],
///     [0.0, 1.0, 0.0],
///     [0.0, 0.0, 1.0],
///     [0.0, 1.0, 0.0],
///     [0.0, 1.0, 0.0],
/// ]);
/// ```
pub fn one_hot_encode<const B: usize, const N: usize>(class_labels: &[usize; B]) -> Tensor2D<B, N> {
    let mut result = Tensor2D::zeros();
    for i in 0..B {
        result.mut_data()[i][class_labels[i]] = 1.0;
    }
    result
}

/// A utility class to simplify sampling a fixed number of indices for
/// data from a dataset.
///
/// Generic Arguments:
/// - `B` - The number of indices to sample for a batch.
///
/// Iterating a dataset in order:
/// ```rust
/// # use dfdx::prelude::*;
/// let mut subsets = SubsetIterator::<5>::in_order(100);
/// assert_eq!(subsets.next(), Some([0, 1, 2, 3, 4]));
/// ```
///
/// Iterating a dataset in random order:
/// ```rust
/// # use dfdx::prelude::*;
/// # use rand::prelude::*;
/// let mut rng = StdRng::seed_from_u64(0);
/// let mut subsets = SubsetIterator::<5>::shuffled(100, &mut rng);
/// assert_eq!(subsets.next(), Some([17, 4, 76, 81, 5]));
/// ```
pub struct SubsetIterator<const B: usize> {
    i: usize,
    indices: Vec<usize>,
}

impl<const B: usize> SubsetIterator<B> {
    pub fn in_order(n: usize) -> Self {
        let mut indices: Vec<usize> = Vec::with_capacity(n);
        for i in 0..n {
            indices.push(i);
        }
        Self { i: 0, indices }
    }

    pub fn shuffled<R: rand::Rng>(n: usize, rng: &mut R) -> Self {
        let mut sampler = Self::in_order(n);
        sampler.indices.shuffle(rng);
        sampler
    }
}

impl<const B: usize> Iterator for SubsetIterator<B> {
    type Item = [usize; B];
    fn next(&mut self) -> Option<Self::Item> {
        if self.indices.len() < B || self.i + B > self.indices.len() {
            None
        } else {
            let mut batch = [0; B];
            batch.copy_from_slice(&self.indices[self.i..self.i + B]);
            self.i += B;
            Some(batch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampler_uses_all() {
        let mut seen: Vec<usize> = Vec::new();
        for batch in SubsetIterator::<5>::in_order(100) {
            seen.extend(batch.iter());
        }
        for i in 0..100 {
            assert!(seen.contains(&i));
        }
    }

    #[test]
    fn sampler_drops_last() {
        let mut seen: Vec<usize> = Vec::new();
        for batch in SubsetIterator::<6>::in_order(100) {
            seen.extend(batch.iter());
        }
        for i in 0..96 {
            assert!(seen.contains(&i));
        }
    }
}

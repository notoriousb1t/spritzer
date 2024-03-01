use std::fmt::Debug;

#[allow(dead_code)] // Used for debugging.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Diff<T>
where
    T: Eq + PartialEq + Debug + Copy,
{
    pub index: usize,
    pub left: Vec<T>,
    pub right: Vec<T>,
}

impl<T> Diff<T>
where
    T: Eq + PartialEq + Debug + Copy,
{
    #[allow(dead_code)]
    pub fn value(index: usize, left: T, right: T) -> Self {
        Self::range(index, vec![left], vec![right])
    }

    #[allow(dead_code)]
    pub fn range(index: usize, left: Vec<T>, right: Vec<T>) -> Self {
        Diff { index, left, right }
    }

    /// Returns diffs for testing and debugging purposes.
    #[allow(dead_code)]
    pub(crate) fn compare(left_inputs: &[T], right_inputs: &[T]) -> Vec<Diff<T>> {
        let mut deltas: Vec<Diff<T>> = vec![];

        let mut diff_index: Option<usize> = None;
        let mut left_result: Vec<T> = vec![];
        let mut right_result: Vec<T> = vec![];

        for (index, value) in left_inputs.iter().enumerate() {
            let left_value = Some(value);
            let right_value = match index < right_inputs.len() {
                true => Some(&right_inputs[index]),
                false => None,
            };

            match diff_index {
                Some(index) => {
                    if right_value != left_value {
                        // Continue collecting values.
                        if let Some(v) = left_value {
                            left_result.push(*v);
                        }
                        if let Some(v) = right_value {
                            right_result.push(*v);
                        }
                        continue;
                    }

                    // If the same value is detected, create a delta and clear all tracking
                    // fields for diffs.
                    deltas.push(Diff::range(index, left_result, right_result));
                    diff_index = None;
                    left_result = vec![];
                    right_result = vec![];
                }
                None => {
                    if right_value != left_value {
                        // Continue until a non-matching value is found.
                        diff_index = Some(index);
                        if let Some(v) = left_value {
                            left_result.push(*v);
                        }
                        if let Some(v) = right_value {
                            right_result.push(*v);
                        }
                    }
                }
            }
        }

        if let Some(address) = diff_index {
            // Add the final delta if there is still an ongoing delta at the end of the vectors.
            deltas.push(Diff::range(address, left_result, right_result));
        }
        if left_inputs.len() < right_inputs.len() {
            // Create new deltas if the right side is larger than the left.
            deltas.push(Diff::range(
                left_inputs.len(),
                vec![],
                right_inputs[left_inputs.len()..].to_vec(),
            ))
        }

        deltas
    }
}

#[cfg(test)]
mod tests {
    use crate::common::diff::Diff;

    #[test]
    fn diff_empty() {
        let diff = Diff::compare(&[0u8], &[0u8]);
        assert_eq!(diff, vec![]);
    }

    #[test]
    fn diff_full() {
        let actual = Diff::compare(&[0xFFu8; 16], &[0u8; 16]);
        let expectation = [Diff::range(0, vec![0xFFu8; 16], vec![0u8; 16])];
        assert_eq!(actual, expectation);
    }

    #[test]
    fn diff_right_larger() {
        let actual = Diff::compare(&[0xFFu8; 2], &[0xFFu8; 4]);
        let expectation = [Diff::range(2, vec![], vec![0xFFu8; 2])];
        assert_eq!(actual, expectation);
    }

    #[test]
    fn diff_left_larger() {
        let actual = Diff::compare(&[0xFFu8; 4], &[0xFFu8; 2]);
        let expectation = [Diff::range(2, vec![0xFFu8; 2], vec![])];
        assert_eq!(actual, expectation);
    }
}

pub trait Indices<T> {
    fn get_indices(&self, indices: &[usize]) -> Vec<Option<&T>>;
    fn get_mut_indices(&mut self, indices: &[usize]) -> Vec<Option<&mut T>>;
}

// impl<T: Copy> Indices<T> for Vec<T> {
//     fn get_indices(&self, indices: &[usize]) -> Vec<Option<&T>> {
//         todo!()
//     }
//
//     fn get_mut_indices(&mut self, indices: &[usize]) -> Vec<Option<&mut T>> {
//         let mut indices = indices.to_owned();
//         indices.sort_unstable();
//
//         let mut output = Vec::new();
//         let to_split = RefCell::new(self.as_mut_slice());
//         for i in 0..indices.len() {
//             let (_, next_slice) = to_split.borrow_mut().split_at_mut(indices[i]);
//             if let Some((value, rest)) = next_slice.split_first_mut() {
//                 output.push(Some(value));
//                 to_split.swap(&RefCell::new(rest))
//             } else {
//                 output.push(None);
//             }
//         }
//
//         output
//     }
// }

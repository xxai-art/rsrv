use ndarray::{ArrayBase, Data, Ix1};
use num_traits::Float;

pub fn norm01<T>(input: &ArrayBase<impl Data<Elem = T>, Ix1>) -> ArrayBase<impl Data<Elem = T>, Ix1>
where
  T: Float + Default,
{
  let min = input
    .iter()
    .fold(T::infinity(), |min, &val| T::min(min, val));
  let max = input
    .iter()
    .fold(T::neg_infinity(), |max, &val| T::max(max, val));
  let range = max - min;
  let zero = T::default();
  if range == zero {
    return input.mapv(|_| zero);
  }
  input.mapv(|val| (val - min) / range)
}

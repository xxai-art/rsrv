use crate::{nd::norm01, ndarray::prelude::arr1};

pub fn sort(id_li: Vec<u64>, score_li: Vec<f32>, quality_li: Vec<f32>) -> Vec<u64> {
  let score_li = norm01(&arr1(&score_li));
  let quality_li = norm01(&arr1(&quality_li));
  let rank_li = &quality_li + &score_li;

  let mut li = rank_li
    .into_iter()
    .enumerate()
    .map(|(pos, rank)| (id_li[pos], rank))
    .collect::<Vec<_>>();

  li.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
  li.into_iter().map(|i| i.0).collect()
}

// use super::Solution;

// impl Solution {
//     pub fn find_ladders(
//         begin_word: String,
//         end_word: String,
//         mut word_list: Vec<String>,
//     ) -> Vec<Vec<String>> {
//         fn backtracing(
//             idx: usize,
//             begin_word_idx: usize,
//             word_list: &[String],
//             records: &[(i32, Vec<usize>)],
//         ) -> Vec<Vec<String>> {
//             let word = &word_list[idx];

//             if idx == begin_word_idx {
//                 return vec![vec![word.to_string()]];
//             }

//             records[idx]
//                 .1
//                 .iter()
//                 .flat_map(|&v| backtracing(v, begin_word_idx, word_list, records))
//                 .map(|mut v| {
//                     v.push(word.to_string());
//                     v
//                 })
//                 .collect()
//         }

//         let (mut begin_word_idx, mut end_word_idx) = (None, None);
//         for (i, word) in word_list.iter().enumerate() {
//             if word == &begin_word {
//                 begin_word_idx = Some(i);
//             } else if word == &end_word {
//                 end_word_idx = Some(i);
//             }
//         }

//         let end_word_idx = match end_word_idx {
//             Some(i) => i,
//             None => return vec![],
//         };

//         let begin_word_idx = match begin_word_idx {
//             Some(i) => i,
//             None => {
//                 word_list.push(begin_word);
//                 word_list.len() - 1
//             }
//         };

//         let mut records = vec![(-1, vec![]); word_list.len()];
//         records[begin_word_idx].0 = 0;

//         for step in 0.. {
//             let mut flag = false;

//             for (i, word1) in word_list.iter().enumerate() {
//                 if records[i].0 == step {
//                     for (j, word2) in word_list.iter().enumerate() {
//                         if (records[j].0 == -1 || records[j].0 == step + 1)
//                             && word1
//                                 .bytes()
//                                 .zip(word2.bytes())
//                                 .fold(0, |diff_count, (b1, b2)| match b1 == b2 {
//                                     true => diff_count,
//                                     false => diff_count + 1,
//                                 })
//                                 == 1
//                         {
//                             records[j].0 = step + 1;
//                             records[j].1.push(i);
//                             flag = true;
//                         }
//                     }
//                 }
//             }

//             if !flag || records[end_word_idx].0 != -1 {
//                 break;
//             }
//         }

//         match records[end_word_idx].0 == -1 {
//             true => vec![],
//             false => backtracing(end_word_idx, begin_word_idx, &word_list[..], &records[..]),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_find_ladders() {
//         assert_eq!(
//             Solution::find_ladders(
//                 "hit".to_string(),
//                 "cog".to_string(),
//                 vec![
//                     "hot".to_string(),
//                     "dot".to_string(),
//                     "dog".to_string(),
//                     "lot".to_string(),
//                     "log".to_string(),
//                     "cog".to_string()
//                 ]
//             ),
//             vec![
//                 vec![
//                     "hit".to_string(),
//                     "hot".to_string(),
//                     "dot".to_string(),
//                     "dog".to_string(),
//                     "cog".to_string()
//                 ],
//                 vec![
//                     "hit".to_string(),
//                     "hot".to_string(),
//                     "lot".to_string(),
//                     "log".to_string(),
//                     "cog".to_string()
//                 ]
//             ]
//         );

//         assert_eq!(
//             Solution::find_ladders(
//                 "hit".to_string(),
//                 "cog".to_string(),
//                 vec![
//                     "hot".to_string(),
//                     "dot".to_string(),
//                     "dog".to_string(),
//                     "lot".to_string(),
//                     "log".to_string()
//                 ]
//             ),
//             Vec::<Vec<String>>::new()
//         );
//     }
// }

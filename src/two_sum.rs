extern crate std;

use std::collections::HashMap;

pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<(i32, i32)> {
    let mut freq: HashMap<i32, i32> = HashMap::new();

    let mut vec: Vec<(i32, i32)> = Vec::new();

    for i in 0..nums.len() {
        let curr = nums[i];

        if let Some(&i) = freq.get(&(target - curr)) {
            if i != 0 {
                vec.push((curr, target - curr));
                freq.insert(target - curr, i - 1);
                if i == 1 {
                    freq.remove(&mut (target - curr));
                }
            }
        } else {
            if let Some(&i) = freq.get(&curr) {
                freq.insert(curr, i + 1);
            } else {
                freq.insert(curr, 1);
            }
        }

    }

    vec

}

#[cfg(test)]
mod tests {

    #[test]
    fn one_pair() {
        let v = vec![2, 7, 11, 15];
        assert_eq!(super::two_sum(&v, 9).get(0), Some(&(7, 2)));
    }

    #[test]
    fn two_pairs() {
        let v = vec![1, 3, 2, 2];
        let v = super::two_sum(&v, 4);

        assert_eq!(v.get(0), Some(&(3, 1)));
        assert_eq!(v.get(1), Some(&(2, 2)));
    }

}

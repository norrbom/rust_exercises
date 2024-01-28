fn solution(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            if numbers[i] + numbers[j] == target {
                return Some((i, j));
            }
        }
    }
    return None;
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            if nums[i] + nums[j] == target {
                println!("{} + {} = {}", nums[i], nums[j], target);
                result.push(i as i32);
                result.push(j as i32);
                return result;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tow_sum() {
        let numbers = [2, 7, 11, 15];
        assert_eq!(solution(&numbers, 9), Some((0, 1)));
        assert_eq!(solution(&numbers, 99), None);
    }
    #[test]
    fn test_tow_sum_vec() {
        let numbers = [3, 2, 4];
        assert_eq!(two_sum(numbers.to_vec(), 6), vec![1, 2]);
    }
}

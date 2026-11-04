pub const BASE: i32 = 5;
const HEIGHT: i32 = 8;

const fn calculate_tower_result_length(height: i32) -> usize {
    (height - 1) as usize * 2
}

type CalculationType = i64;
type TowerResult = [(CalculationType, char, i32, CalculationType); calculate_tower_result_length(HEIGHT)];

pub fn calculate_tower(base: i32) -> TowerResult {
    let mut result_array: TowerResult = [(0, ' ', 0, 0); calculate_tower_result_length(HEIGHT)];

    let mut index = 0;
    let mut i = base as CalculationType;
    for multiplier in 2..=HEIGHT {
        let result = i * multiplier as CalculationType;
        result_array[index] = (i, '*', multiplier, result);
        index += 1;
        i = result;
    }
    for divisor in 2..=HEIGHT {
        let result = i / divisor as CalculationType;
        result_array[index] = (i, '/', divisor, result);
        index += 1;
        i = result;
    }

    result_array
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tower() {
        let result = calculate_tower(2);
        assert_eq!(result[0].0, result[result.len() - 1].3);
    }
}

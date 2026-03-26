use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;
use wasm_bindgen::prelude::*;

pub fn fix_dim(row_sums: &[i32], column_sums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let row_len = row_sums.len() as i32;
    let col_len = column_sums.len() as i32;

    let fixed_rows = row_sums.iter().map(|&val| val.min(col_len)).collect();

    let fixed_cols = column_sums.iter().map(|&val| val.min(row_len)).collect();

    (fixed_rows, fixed_cols)
}

pub fn fix_balance(row_sums: &[i32], column_sums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut fix_row_sums: Vec<i32> = row_sums.to_vec();
    let mut fix_col_sums: Vec<i32> = column_sums.to_vec();
    let total_row_sum: i32 = fix_row_sums.iter().sum();
    let total_col_sum: i32 = fix_col_sums.iter().sum();

    if total_col_sum < total_row_sum {
        let mut deficit: i32 = total_row_sum - total_col_sum;
        let row_len: i32 = fix_row_sums.len() as i32;

        while deficit > 0 {
            let new_col = std::cmp::min(deficit, row_len);
            fix_col_sums.push(new_col);
            deficit -= new_col;
        }
    }

    if total_row_sum < total_col_sum {
        let mut deficit: i32 = total_col_sum - total_row_sum;
        let col_len: i32 = fix_col_sums.len() as i32;

        while deficit > 0 {
            let new_row = std::cmp::min(deficit, col_len);
            fix_row_sums.push(new_row);
            deficit -= new_row;
        }
    }

    (fix_row_sums, fix_col_sums)
}

pub fn fix_majorization(row_sums: &[i32], column_sums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut fix_row_sums: Vec<i32> = row_sums.to_vec();
    let mut fix_col_sums: Vec<i32> = column_sums.to_vec();

    fix_row_sums.sort_by(|a, b| b.cmp(a));
    fix_col_sums.sort_by(|a, b| b.cmp(a));

    let row_len: usize = fix_row_sums.len();
    let col_len: usize = fix_col_sums.len();

    let mut ferrers_matrix = vec![vec![0; col_len]; row_len];

    for i in 0..row_len {
        let mut num_ones = row_sums[i];
        for j in 0..col_len {
            if num_ones > 0 {
                ferrers_matrix[i][j] = 1;
                num_ones -= 1;
            }
        }
    }

    let conjugate_sums = get_column_sums(&ferrers_matrix);
    let l = std::cmp::max(col_len, conjugate_sums.len());

    let mut prefix_conj_sums = 0;
    let mut prefix_col_sums = 0;

    for i in 0..l {
        if i >= conjugate_sums.len() {
            prefix_conj_sums += 0;
        } else {
            prefix_conj_sums += conjugate_sums[i];
        }

        if i >= col_len {
            prefix_col_sums += 0;
        } else {
            prefix_col_sums += fix_col_sums[i];
        }

        if prefix_conj_sums < prefix_col_sums {
            let diff = prefix_col_sums - prefix_conj_sums;
            fix_col_sums[i] -= diff;

            if i + 1 < fix_col_sums.len() {
                fix_col_sums[i + 1] += diff;
            }
            prefix_col_sums -= diff;
        }
    }

    (fix_row_sums, fix_col_sums)
}

pub fn total_fix(row_sums: &[i32], column_sums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let (rows, cols) = fix_dim(row_sums, column_sums);
    let (mut rows, mut cols) = fix_balance(&rows, &cols);
    rows.sort_by(|a, b| b.cmp(a));
    cols.sort_by(|a, b| b.cmp(a));
    let (rows, cols) = fix_majorization(&rows, &cols);

    (rows, cols)
}

pub fn total_fix_preserved(row_sums: &[i32], column_sums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    // 1. Tag inputs with their original indices
    let mut row_tuples: Vec<(usize, i32)> = row_sums.iter().copied().enumerate().collect();
    let mut col_tuples: Vec<(usize, i32)> = column_sums.iter().copied().enumerate().collect();

    // 2. Sort descending by value
    row_tuples.sort_by(|a, b| b.1.cmp(&a.1));
    col_tuples.sort_by(|a, b| b.1.cmp(&a.1));

    let sorted_rows: Vec<i32> = row_tuples.iter().map(|t| t.1).collect();
    let sorted_cols: Vec<i32> = col_tuples.iter().map(|t| t.1).collect();

    // 3. Run Step 1 & 2 of the math pipeline
    let (f_rows, f_cols) = fix_dim(&sorted_rows, &sorted_cols);
    let (f_rows, f_cols) = fix_balance(&f_rows, &f_cols);

    // 4. fix_balance might have appended elements. We must assign them new indices
    // and re-sort before the Gale-Ryser majorization
    let mut current_row_tuples = Vec::new();
    for i in 0..f_rows.len() {
        let original_idx = if i < row_tuples.len() {
            row_tuples[i].0
        } else {
            row_sums.len() + (i - row_tuples.len())
        };
        current_row_tuples.push((original_idx, f_rows[i]));
    }

    let mut current_col_tuples = Vec::new();
    for i in 0..f_cols.len() {
        let original_idx = if i < col_tuples.len() {
            col_tuples[i].0
        } else {
            column_sums.len() + (i - col_tuples.len())
        };
        current_col_tuples.push((original_idx, f_cols[i]));
    }

    // Re-sort descending before majorization
    current_row_tuples.sort_by(|a, b| b.1.cmp(&a.1));
    current_col_tuples.sort_by(|a, b| b.1.cmp(&a.1));

    let maj_input_rows: Vec<i32> = current_row_tuples.iter().map(|t| t.1).collect();
    let maj_input_cols: Vec<i32> = current_col_tuples.iter().map(|t| t.1).collect();

    // 5. Run Step 3: Gale-Ryser Majorization
    let (final_fixed_rows, final_fixed_cols) = fix_majorization(&maj_input_rows, &maj_input_cols);

    // 6. Map the mathematically fixed values back to their tracked indices
    for i in 0..final_fixed_rows.len() {
        current_row_tuples[i].1 = final_fixed_rows[i];
    }
    for i in 0..final_fixed_cols.len() {
        current_col_tuples[i].1 = final_fixed_cols[i];
    }

    // 7. Un-sort: Sort ascending by the ORIGINAL index to restore the visual order
    current_row_tuples.sort_by(|a, b| a.0.cmp(&b.0));
    current_col_tuples.sort_by(|a, b| a.0.cmp(&b.0));

    let restored_rows: Vec<i32> = current_row_tuples.into_iter().map(|t| t.1).collect();
    let restored_cols: Vec<i32> = current_col_tuples.into_iter().map(|t| t.1).collect();

    (restored_rows, restored_cols)
}

fn get_column_sums(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let row_len: usize = matrix.len();
    let col_len: usize = matrix[0].len();
    let mut column_sums: Vec<i32> = vec![];
    for i in 0..col_len {
        let mut sum = 0;
        for j in 0..row_len {
            sum += matrix[j][i]
        }
        column_sums.push(sum);
    }
    column_sums
}

fn get_row_sums(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let row_len: usize = matrix.len();
    let col_len: usize = matrix[0].len();
    let mut row_sums: Vec<i32> = vec![];
    for i in 0..row_len {
        let mut sum = 0;
        for j in 0..col_len {
            sum += matrix[i][j]
        }
        row_sums.push(sum);
    }
    row_sums
}

fn is_matrix_possible(row_sums: &[i32], column_sums: &[i32]) -> bool {
    let row_len: usize = row_sums.len();
    let col_len: usize = column_sums.len();
    let mut sorted_rows = row_sums.to_vec();
    let mut sorted_cols = column_sums.to_vec();

    sorted_rows.sort_by(|a, b| b.cmp(a));
    sorted_cols.sort_by(|a, b| b.cmp(a));

    let mut ferrers_matrix = vec![vec![0; col_len]; row_len];

    for i in 0..row_len {
        let mut num_ones = sorted_rows[i];
        for j in 0..col_len {
            if num_ones > 0 {
                ferrers_matrix[i][j] = 1;
                num_ones -= 1;
            }
        }
    }

    let conjugate_sums: Vec<i32> = get_column_sums(&ferrers_matrix);
    let conj_len: usize = conjugate_sums.len();
    let l = cmp::max(col_len, conj_len);

    let mut prefix_conj_sums = 0;
    let mut prefix_col_sums = 0;

    for i in 0..l {
        if i >= conj_len {
            prefix_conj_sums += 0;
        } else {
            prefix_conj_sums += conjugate_sums[i];
        }

        if i >= col_len {
            prefix_col_sums += 0;
        } else {
            prefix_col_sums += sorted_cols[i];
        }

        if prefix_conj_sums < prefix_col_sums {
            return false;
        }
    }

    true
}

fn generate_matrix_for(row_sums: &[i32], column_sums: &[i32]) -> Result<Vec<Vec<i32>>, String> {
    let row_len = row_sums.len();
    let col_len = column_sums.len();
    let mut ferrers_matrix = vec![vec![0; col_len]; row_len];

    if !is_matrix_possible(row_sums, column_sums) {
        return Err(String::from("Matrix failed majorization check.\n"));
    } else {
        for i in 0..row_len {
            let mut num_ones = row_sums[i];
            for j in 0..col_len {
                if num_ones > 0 {
                    ferrers_matrix[i][j] = 1;
                    num_ones -= 1;
                }
            }
        }

        let mut current_col_sums = get_column_sums(&ferrers_matrix);

        // --- CIRCUIT BREAKER 1: Stop the Outer Loop ---
        let mut max_iters = 10_000;

        while current_col_sums != *column_sums {
            // Check outer circuit breaker
            if max_iters == 0 {
                return Err(String::from(
                    "Generation timed out. The matrix might be mathematically impossible without sorting.",
                ));
            }
            max_iters -= 1;

            let mut excess_columns: Vec<usize> = vec![];
            let mut deficient_columns: Vec<usize> = vec![];
            let mut flag: bool = false;

            for i in 0..col_len {
                if current_col_sums[i] > column_sums[i] {
                    excess_columns.push(i);
                    flag = true;
                }
            }

            if !flag {
                return Err(String::from(
                    "Could not find any columns with excess sum.\n",
                ));
            }

            flag = false;
            for i in 0..col_len {
                if current_col_sums[i] < column_sums[i] {
                    deficient_columns.push(i);
                    flag = true;
                }
            }

            if !flag {
                return Err(String::from(
                    "Could not find any columns with deficient sum.\n",
                ));
            }

            let mut rng = thread_rng();
            let mut possible_row_switch: Vec<usize> = vec![];
            let mut i = 0;
            let mut j = 0;

            // --- CIRCUIT BREAKER 2: Stop the Inner Swap Search ---
            let mut swap_iters = 1_000;

            while possible_row_switch.is_empty() {
                // Check inner circuit breaker
                if swap_iters == 0 {
                    return Err(String::from(
                        "Deadlock! The randomizer got trapped and could not find a valid row to swap. Try again.",
                    ));
                }
                swap_iters -= 1;

                // Safely unwrap here because the error returns above if it runs out of tries
                i = *excess_columns.choose(&mut rng).unwrap();
                j = *deficient_columns.choose(&mut rng).unwrap();

                for row in 0..row_len {
                    if ferrers_matrix[row][i] == 1 && ferrers_matrix[row][j] == 0 {
                        possible_row_switch.push(row);
                    }
                }
            }

            let row = *possible_row_switch.choose(&mut rng).unwrap();

            ferrers_matrix[row][i] = 0;
            ferrers_matrix[row][j] = 1;

            current_col_sums = get_column_sums(&ferrers_matrix);
        }
    };

    Ok(ferrers_matrix)
}

#[wasm_bindgen]
pub fn generate_matrix_wasm(
    row_sums: &[i32],
    column_sums: &[i32],
    fix: bool,
) -> Result<JsValue, JsValue> {
    let final_rows: Vec<i32>;
    let final_cols: Vec<i32>;

    if fix {
        let (fixed_rows, fixed_cols) = total_fix_preserved(row_sums, column_sums);
        final_rows = fixed_rows;
        final_cols = fixed_cols;
    } else {
        final_rows = row_sums.to_vec();
        final_cols = column_sums.to_vec();
    }

    let matrix = generate_matrix_for(&final_rows, &final_cols);
    match matrix {
        Ok(matrix) => {
            return Ok(serde_wasm_bindgen::to_value(&matrix).unwrap());
        }

        Err(error_message) => {
            return Err(JsValue::from_str(&format!(
                "Matrix was not generated: {}",
                &error_message
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_infinite_loop() {
        let rows = vec![0, 3, 2, 0, 1, 2, 2];
        let cols = vec![1, 1, 1];

        let (fixed_rows, fixed_cols) = total_fix(&rows, &cols);

        println!("--- State Before Generation ---");
        println!("Fixed Rows: {:?}", fixed_rows);
        println!("Fixed Cols: {:?}", fixed_cols);
        println!("-------------------------------");

        let _ = generate_matrix_for(&fixed_rows, &fixed_cols);
    }
}

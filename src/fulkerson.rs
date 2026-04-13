// src/fulkerson.rs

use rand::seq::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::*;

fn get_column_sums(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let mut column_sums = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            column_sums[j] += matrix[i][j];
        }
    }
    column_sums
}

fn get_fulkerson_conjugate_sum(row_sums: &[i32], k: usize) -> i32 {
    let n = row_sums.len();
    let mut conjugate_sum = 0;

    for i in 0..n {
        if i < k {
            if row_sums[i] >= k as i32 {
                conjugate_sum += 1;
            }
        } else if i > k {
            if row_sums[i] >= (k as i32) + 1 {
                conjugate_sum += 1;
            }
        }
    }
    conjugate_sum
}

fn is_zero_trace_possible(row_sums: &[i32], column_sums: &[i32]) -> bool {
    let n = row_sums.len();
    if n != column_sums.len() {
        return false;
    }

    let mut sorted_rows = row_sums.to_vec();
    let mut sorted_cols = column_sums.to_vec();
    sorted_rows.sort_by(|a, b| b.cmp(a));
    sorted_cols.sort_by(|a, b| b.cmp(a));

    let mut running_col_sum = 0;
    let mut running_conj_sum = 0;

    for k in 0..n {
        running_col_sum += sorted_cols[k];
        running_conj_sum += get_fulkerson_conjugate_sum(&sorted_rows, k);

        if running_col_sum > running_conj_sum {
            return false;
        }
    }

    let total_r: i32 = sorted_rows.iter().sum();
    let total_c: i32 = sorted_cols.iter().sum();
    if total_r != total_c {
        return false;
    }

    true
}

fn generate_fulkerson_matrix(
    row_sums: &[i32],
    column_sums: &[i32],
) -> Result<Vec<Vec<i32>>, String> {
    let n = row_sums.len();

    if !is_zero_trace_possible(row_sums, column_sums) {
        return Err(String::from(
            "Matrix failed Fulkerson zero-trace majorization check.\n",
        ));
    }

    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        let mut num_ones = row_sums[i];
        for j in 0..n {
            if num_ones > 0 && i != j {
                matrix[i][j] = 1;
                num_ones -= 1;
            }
        }
    }

    let mut current_col_sums = get_column_sums(&matrix);
    let mut max_iters = 10_000;

    while current_col_sums != *column_sums {
        if max_iters == 0 {
            return Err(String::from(
                "Generation timed out. The matrix might be mathematically impossible without sorting.",
            ));
        }
        max_iters -= 1;

        let mut excess_cols = Vec::new();
        let mut deficient_cols = Vec::new();

        for i in 0..n {
            if current_col_sums[i] > column_sums[i] {
                excess_cols.push(i);
            }
            if current_col_sums[i] < column_sums[i] {
                deficient_cols.push(i);
            }
        }

        if excess_cols.is_empty() || deficient_cols.is_empty() {
            return Err(String::from(
                "Math mismatch: Columns do not balance correctly.",
            ));
        }

        let mut rng = thread_rng();
        let mut possible_row_switch: Vec<usize> = Vec::new();

        let mut swap_iters = 1_000;
        let mut i = 0;
        let mut j = 0;

        while possible_row_switch.is_empty() {
            if swap_iters == 0 {
                return Err(String::from(
                    "Deadlock! The randomizer got trapped and could not find a valid zero-trace swap.",
                ));
            }
            swap_iters -= 1;

            i = *excess_cols.choose(&mut rng).unwrap();
            j = *deficient_cols.choose(&mut rng).unwrap();

            for row in 0..n {
                if matrix[row][i] == 1 && matrix[row][j] == 0 && row != i && row != j {
                    possible_row_switch.push(row);
                }
            }
        }

        let row = *possible_row_switch.choose(&mut rng).unwrap();
        matrix[row][i] = 0;
        matrix[row][j] = 1;

        current_col_sums = get_column_sums(&matrix);
    }

    Ok(matrix)
}

// The pub function exported to JavaScript
#[wasm_bindgen]
pub fn generate_fulkerson_wasm(row_sums: &[i32], column_sums: &[i32]) -> Result<JsValue, JsValue> {
    if row_sums.len() != column_sums.len() {
        return Err(JsValue::from_str(
            "Fulkerson matrices require the same number of rows and columns (a square matrix).",
        ));
    }

    match generate_fulkerson_matrix(row_sums, column_sums) {
        Ok(matrix) => Ok(serde_wasm_bindgen::to_value(&matrix).unwrap()),
        Err(error_message) => Err(JsValue::from_str(&format!(
            "Matrix was not generated: {}",
            &error_message
        ))),
    }
}

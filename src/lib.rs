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
    let fix_row_sums: Vec<i32> = row_sums.to_vec();
    let mut fix_col_sums: Vec<i32> = column_sums.to_vec();

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
        }
        else {
            prefix_conj_sums += conjugate_sums[i];
        }

        if i >= col_len {
            prefix_col_sums += 0;
        }
        else {
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
    let (rows, cols) =  fix_dim(row_sums, column_sums);
    let (rows, cols) = fix_balance(&rows, &cols);
    let (rows, cols) = fix_majorization(&rows, &cols);

    (rows, cols)
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
            prefix_col_sums += column_sums[i];
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
        while current_col_sums != *column_sums {
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
            let i = *excess_columns.choose(&mut rng).expect("No columns found.");
            let j = *deficient_columns
                .choose(&mut rng)
                .expect("No columns found.");

            let mut possible_row_switch: Vec<usize> = vec![];

            for row in 0..row_len {
                if ferrers_matrix[row][i] == 1 && ferrers_matrix[row][j] == 0 {
                    possible_row_switch.push(row);
                }
            }

            let row = *possible_row_switch
                .choose(&mut rng)
                .expect("No possible row switchings");

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
    fix: bool
) -> Result<JsValue, JsValue> {
    let final_rows: Vec<i32>;
    let final_cols: Vec<i32>;

    if fix {
        let (fixed_rows, fixed_cols) = total_fix(row_sums, column_sums);
        final_rows = fixed_rows;
        final_cols = fixed_cols;
    }
    else {
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

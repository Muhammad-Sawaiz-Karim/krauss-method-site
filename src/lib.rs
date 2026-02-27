use std::cmp;
use rand::thread_rng;
use rand::seq::SliceRandom;

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

fn is_matrix_possible(row_sums: &Vec<i32>, column_sums: &Vec<i32>) -> bool {
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

fn generate_matrix_for(row_sums: &Vec<i32>, column_sums: &Vec<i32>) -> Result<Vec<Vec<i32>>, String> {
    let row_len = row_sums.len();
    let col_len = column_sums.len();
    let mut ferrers_matrix = vec![vec![0; col_len]; row_len];
    
    if !is_matrix_possible(row_sums, column_sums) {
        return Err(String::from("Matrix failed majorization check.\n"));
    } else {
        
        for i in 0..row_len
        {
            let mut num_ones = row_sums[i];
            for j in 0..col_len
            {
                if num_ones > 0
                {
                    ferrers_matrix[i][j] = 1;
                    num_ones -= 1;
                }
            }
        }

        let mut current_col_sums = get_column_sums(&ferrers_matrix);
        while current_col_sums != *column_sums
        {
            let mut excess_columns: Vec<usize> = vec![];
            let mut deficient_columns: Vec<usize> = vec![];
            let mut flag: bool = false;

            for i in 0..col_len
            {
                if current_col_sums[i] > column_sums[i]
                {
                    excess_columns.push(i);
                    flag = true;
                }
            }
            
            if !flag
            {
                return Err(String::from("Could not find any columns with excess sum.\n"));
            }

            flag = false;
            for i in 0..col_len
            {
                if current_col_sums[i] < column_sums[i]
                {
                    deficient_columns.push(i);
                    flag = true;
                }
            }

            if !flag
            {
                return Err(String::from("Could not find any columns with deficient sum.\n"));
            }
            let mut rng = thread_rng();
            let i = *excess_columns.choose(&mut rng).expect("No columns found.");
            let j = *deficient_columns.choose(&mut rng).expect("No columns found.");
            
            let mut possible_row_switch: Vec<usize> = vec![];
            
            for row in 0..row_len
            {
                if ferrers_matrix[row][i] == 1 && ferrers_matrix[row][j] == 0
                {
                    possible_row_switch.push(row);
                }
            }

            let row = *possible_row_switch.choose(&mut rng).expect("No possible row switchings");

            ferrers_matrix[row][i] = 0;
            ferrers_matrix[row][j] = 1;

            current_col_sums = get_column_sums(&ferrers_matrix);
        }
    };

    Ok(ferrers_matrix)
}
pub fn count_cross_mas(char_matrix: &Vec<Vec<char>>) -> usize {
    let mut mas_count = 0;

    for row in 1..char_matrix[0].len() - 1 {
        for column in 1..char_matrix.len() - 1 {
            if is_cross_mas(&char_matrix, row, column) {
                mas_count += 1;
            }
        }
    }

    mas_count
}

fn is_cross_mas(
    char_matrix: &Vec<Vec<char>>,
    row: usize,
    column: usize
) -> bool {
    if char_matrix[row][column] != 'A' {
        return false;
    }

    let corner_chars = format!(
        "{}{}{}{}",
        char_matrix[row - 1][column - 1],
        char_matrix[row - 1][column + 1],
        char_matrix[row + 1][column - 1],
        char_matrix[row + 1][column + 1]
    );
    ["MMSS", "SSMM", "MSMS", "SMSM"].contains(&corner_chars.as_str())
}

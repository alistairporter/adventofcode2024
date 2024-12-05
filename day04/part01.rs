const TARGET_WORD: &str = "XMAS";

pub fn count_xmas(char_matrix: &Vec<Vec<char>> ) -> usize {
    let mut word_count = 0;
    for row in 0..char_matrix[0].len() {
        for column in 0..char_matrix.len() {
            let cur_char = char_matrix[row][column];
            if cur_char == 'X' {
                word_count += count_words(&char_matrix, row, column);
            }
        }
    }
    return word_count;
}

fn count_words(
    char_matrix: &Vec<Vec<char>>,
    row: usize,
    column: usize
) -> usize {
    let length_row = char_matrix[0].len();
    let length_column = char_matrix.len();

    let mut count = 0;

    for direction_row in -1..=1 {
        for direction_column in -1..=1 {
            let potential_word = 
                get_possible_word(
                    char_matrix,
                    length_row,
                    length_column,
                    row,
                    column,
                    direction_row,
                    direction_column,
                    4
                );
            
            if potential_word == TARGET_WORD {
                count += 1;
            }
        }
    }
    count
}

fn get_possible_word(
    char_matrix: &Vec<Vec<char>>,
    length_row: usize,
    length_column: usize,
    current_row: usize,
    current_col: usize,
    direction_row: isize,
    direction_column: isize,
    number_of_chars: usize,
) -> String {
    let mut word = String::new();
    
    let mut new_row: isize = current_row as isize;
    let mut new_column: isize = current_col as isize;
    
    for _ in 0..number_of_chars {
        word.push(char_matrix[new_row as usize][new_column as usize]);
        
        new_row = new_row + direction_row;
        new_column = new_column + direction_column;

        if new_row == length_row as isize || new_column == length_column as isize || new_row < 0 || new_column < 0
        {
            return word;
        }
    }
    word
}

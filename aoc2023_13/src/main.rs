use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pattern = Vec::<u8>::new();
    let mut width = 0;

    let mut total = 0;

    for line in stdin().lines() {
        let line = line?;

        let bytes = line.as_bytes();

        if bytes.is_empty() {
            let orig_answer = process_pattern(&pattern, width, 0);
            for smudge_pos in 0..pattern.len() {
                let orig = pattern[smudge_pos];
                pattern[smudge_pos] = if orig == b'.' { b'#' } else { b'.' };
                let answer = process_pattern(&pattern, width, orig_answer);
                if answer != 0 && answer != orig_answer {
                    total += answer;
                    break;
                }
                pattern[smudge_pos] = orig;
            }

            pattern.clear();
            width = 0;
        }

        if width == 0 {
            width = bytes.len();
        }

        pattern.extend(bytes);
    }

    if width != 0 {
        let orig_answer = process_pattern(&pattern, width, 0);
        for smudge_pos in 0..pattern.len() {
            let orig = pattern[smudge_pos];
            pattern[smudge_pos] = if orig == b'.' { b'#' } else { b'.' };
            let answer = process_pattern(&pattern, width, orig_answer);
            if answer != 0 && answer != orig_answer {
                total += answer;
                break;
            }
            pattern[smudge_pos] = orig;
        }

        pattern.clear();
        width = 0;
    }

    println!("{}", total);

    Ok(())
}

fn process_pattern(pattern: &[u8], width: usize, exclude: usize) -> usize {
    for row in 0..pattern.len() / width - 1 {
        if test_row(pattern, width, row) {
            let ret = 100 * (row + 1);
            if ret != exclude {
                return ret;
            }
        }
    }
    for col in 0..width - 1 {
        if test_col(pattern, width, col) {
            let ret = col + 1;
            if ret != exclude {
                return ret;
            }
        }
    }
    0
}

fn test_col(pattern: &[u8], width: usize, col: usize) -> bool {
    // refers to gap below row
    for right_col in col + 1..width {
        if right_col - col - 1 > col {
            break;
        }
        let left_col = col - (right_col - col - 1);
        for row in 0..pattern.len() / width {
            if pattern[row * width + left_col] != pattern[row * width + right_col] {
                return false;
            }
        }
    }
    true
}

fn test_row(pattern: &[u8], width: usize, row: usize) -> bool {
    //eprintln!("test_row {}", row);
    // refers to gap below row
    for bottom_row in row + 1..pattern.len() / width {
        if bottom_row - row - 1 > row {
            break;
        }
        let top_row = row - (bottom_row - row - 1);
        //eprintln!(
        //    "test_row row {} top_row {} bottom_row {}",
        //    row, top_row, bottom_row
        //);
        if pattern[top_row * width..(top_row + 1) * width]
            != pattern[bottom_row * width..(bottom_row + 1) * width]
        {
            return false;
        }
    }
    //eprintln!("test_row true");
    true
}

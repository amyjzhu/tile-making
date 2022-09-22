use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // println!("{:?}", score_discrepancy("1111444433331111".to_string()));

    // println!("{}", guess_colour_from_top_down (&"111111111".to_string()));
    // println!("{}", guess_colour_from_bottom_up(&"222222222".to_string()));
    // println!("{}", guess_colour_from_top_down (&"222222222".to_string()));
    // println!("{}", guess_colour_from_bottom_up(&"333333333".to_string()));
    // println!("{}", guess_colour_from_top_down (&"333333333".to_string()));
    // println!("{}", guess_colour_from_bottom_up(&"444444444".to_string()));
    // println!("{}", guess_colour_from_top_down (&"444444444".to_string()));

    let f = File::create("3-unique.json").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all("[".as_bytes()).expect("Unable to write data");

    let mut pattern = [0; 16];
    let mut buf = [0; 16];

    macro_rules! nested_loop {
        ([], $e:expr) => {
            $e
        };
        ([$head:expr, $($tail:expr,)*], $e:expr) => {
            for value in [1, 2, 3, 4] {
                pattern[$head] = value;
                nested_loop!([$($tail,)*], $e);
            }
        };
    }

    let mut idx: u64 = 0;

    nested_loop!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,], {
        let interval = 1000000;
        idx += 1;
        if idx % interval == 0 {
            println!("On index {} million", (idx / interval) as i64);
        }
        // println!("{i:?}");
        let score = score_discrepancy(&pattern, &mut buf);
        // println!("{} with score {}, {}, {}", pattern, score.0, score.1, score.2);
        if score.3.abs() > 0.33 {
            let str = format!(
                "{{'d1': {}, 'd2': {}, 'd3': {}, 'disc': {}, 'pat': {}}},",
                score.0,
                score.1,
                score.2,
                score.3,
                Pattern(pattern)
            );
            // print!("{}", score.3);
            f.write_all(str.as_bytes()).expect("Unable to write data");
        }
    });

    // print!("{}", score.1);
    f.write_all("]".as_bytes()).expect("Unable to write data");
}

struct Pattern([u8; 16]);

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{}", i)?;
        }
        Ok(())
    }
}

fn score_discrepancy(pattern: &[u8], buf: &mut [u8]) -> (f32, f32, f32, f32) {
    let colour1 = guess_colour_from_bottom_up(pattern);
    let colour2 = guess_colour_from_top_down(pattern, buf);
    let colour3 = guess_colour_from_side(pattern, buf);
    (colour1, colour2, colour3, (colour1 - colour2).abs())
}

fn guess_colour_from_side(pattern: &[u8], buf: &mut [u8]) -> f32 {
    // let n = (pattern.len() as f32).sqrt() as usize;
    let n = 4;
    assert_eq!(pattern.len(), n * n);
    for i in 0..n {
        for j in 0..n {
            buf[j * n + i] = pattern[i * 3 + j];
        }
    }

    evaluate_pattern(buf)
}

fn guess_colour_from_bottom_up(pattern: &[u8]) -> f32 {
    evaluate_pattern(pattern)
}

fn guess_colour_from_top_down(pattern: &[u8], buf: &mut [u8]) -> f32 {
    // again this only works because code points will equal chars here, no funny business
    assert_eq!(pattern.len(), buf.len());
    for (i, c) in pattern.iter().enumerate() {
        buf[pattern.len() - i - 1] = *c;
    }

    // println!("pat was {} and new is {}", pattern, pattern_view);
    evaluate_pattern(buf)
}

fn evaluate_pattern(pattern: &[u8]) -> f32 {
    let tile_size = 4;
    let mut score: usize = 0;

    for i in 0..tile_size {
        for j in 0..tile_size {
            // if first row, take from the back to check
            if i == 0 {
                let prev = pattern[(tile_size - 1) * tile_size + j] as usize;
                if prev == 3 || prev == 4 {
                    score += prev - 2;
                } else {
                    let col = pattern[i * tile_size + j] as usize;
                    if col == 3 || col == 4 {
                        score += col - 2;
                    } else {
                        score += col;
                    }
                }
            }
            // # make sure we aren't overriding anything
            //         if pattern_expanded[i][j] == 0 {
            //             pattern_expanded[i][j] = if col % 2 == 0 { 2 } else { 1 };
            //         }

            //         if col == 3 || col == 4 {
            //             raised_ref[i][j] = true;
            //             if i == tile_size - 1 {
            //                 pattern_expanded[0][j] = if col % 2 == 0 { 2 } else { 1 };
            //                 raised_ref[0][j] = true
            //             } else {
            //                 pattern_expanded[i+1][j] = if col % 2 == 0 { 2 } else { 1 };
            //                 raised_ref[i+1][j] = true
            //             }
            //         }
        }
    }

    // let mut pattern_adjusted = vec![vec![0.0; tile_size]; tile_size];

    // for i in 0..tile_size {
    //     for j in 0..tile_size {
    //         pattern_adjusted[i][j] = move_towards_other_colour(pattern_expanded[i][j], adjust_colour_with_neighbors(i, j, &pattern_expanded, &raised_ref));
    //     }
    // }

    // // average
    // let mut score = 0.0;
    // for i in 0..tile_size{
    //     for j in 0..tile_size {
    //         score += pattern_adjusted[i][j];
    //     }
    // }

    // # false_rows = sum([sum([1 if stitch == False  else 0  for stitch in row]) for row in pattern_adjusted])
    score as f32 / (tile_size * tile_size) as f32
}

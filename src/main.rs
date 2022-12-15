use rayon::prelude::*;
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

    let types = [1, 2, 3, 4u8];

    macro_rules! nested_loop {
        ($pattern:ident, [], $e:expr) => {
            $e
        };
        ($pattern:ident, [$head:expr, $($tail:expr,)*], $e:expr) => {
            for value in types {
                $pattern.0[$head] = value;
                nested_loop!($pattern, [$($tail,)*], $e);
            }
        };
    }

    let mut prefixes = vec![];
    for a in types {
        for b in types {
            prefixes.push([a, b]);
        }
    }

    const MAX_PATTERNS: usize = 10_000;

    let kept_patterns: Vec<(Pattern, Score)> = prefixes
        .par_iter()
        .flat_map(|prefix| {
            let mut pattern = Pattern([0; 16]);
            pattern.0[..2].copy_from_slice(prefix);

            let mut buf = Pattern([0; 16]);
            let mut idx = 0;

            let mut keep_patterns = vec![];

            nested_loop!(
                pattern,
                [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,],
                {
                    let interval = 10_000_000;
                    idx += 1;
                    if idx % interval == 0 {
                        println!(
                            "Prefix: {prefix:?} on index {}0 million",
                            (idx / interval) as i64
                        );
                    }
                    // println!("{i:?}");
                    let score = score_discrepancy(&pattern, &mut buf);
                    // println!("{} with score {}, {}, {}", pattern, score.0, score.1, score.2);
                    if score.3.abs() > 0.2 {
                        keep_patterns.push((pattern, score));
                        if keep_patterns.len() > MAX_PATTERNS * 2 {
                            println!("HIT MAX_PATTERNS * 2, truncating...");
                            // sort by score.3, reversed, so biggest score is first
                            keep_patterns.sort_unstable_by(|a, b| {
                                a.1 .3.partial_cmp(&b.1 .3).unwrap().reverse()
                            });
                            keep_patterns.truncate(MAX_PATTERNS);
                        }
                        // let str = format!(
                        //     "{{'d1': {}, 'd2': {}, 'd3': {}, 'disc': {}, 'pat': {}}},",
                        //     score.0,
                        //     score.1,
                        //     score.2,
                        //     score.3,
                        //     Pattern(pattern)
                        // );
                        // print!("{}", score.3);
                        // f.write_all(str.as_bytes()).expect("Unable to write data");
                    }
                }
            );

            keep_patterns.truncate(MAX_PATTERNS);
            keep_patterns
        })
        .collect();

    println!("Found {} patterns", kept_patterns.len());

    // print!("{}", score.1);
    let f = File::create("3-unique.json").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all("[".as_bytes()).expect("Unable to write data");
    // TODO
    f.write_all("]".as_bytes()).expect("Unable to write data");
}

#[derive(Clone, Copy)]
struct Pattern([u8; 16]);

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{}", i)?;
        }
        Ok(())
    }
}

struct Score(f32, f32, f32, f32);

fn score_discrepancy(pattern: &Pattern, buf: &mut Pattern) -> Score {
    let colour1 = guess_colour_from_bottom_up(pattern);
    let colour2 = guess_colour_from_top_down(pattern, buf);
    let colour3 = guess_colour_from_side(pattern, buf);
    Score(colour1, colour2, colour3, (colour1 - colour2).abs())
}

fn guess_colour_from_side(pattern: &Pattern, buf: &mut Pattern) -> f32 {
    // let n = (pattern.len() as f32).sqrt() as usize;
    let n = 4;
    assert_eq!(pattern.0.len(), n * n);
    for i in 0..n {
        for j in 0..n {
            buf.0[j * n + i] = pattern.0[i * 3 + j];
        }
    }

    evaluate_pattern(buf)
}

fn guess_colour_from_bottom_up(pattern: &Pattern) -> f32 {
    evaluate_pattern(pattern)
}

fn guess_colour_from_top_down(pattern: &Pattern, buf: &mut Pattern) -> f32 {
    // again this only works because code points will equal chars here, no funny business
    for (i, c) in pattern.0.iter().enumerate() {
        buf.0[pattern.0.len() - i - 1] = *c;
    }

    // println!("pat was {} and new is {}", pattern, pattern_view);
    evaluate_pattern(buf)
}

fn evaluate_pattern(pattern: &Pattern) -> f32 {
    let tile_size = 4;
    let mut score: usize = 0;

    for i in 0..tile_size {
        for j in 0..tile_size {
            // if first row, take from the back to check
            if i == 0 {
                let prev = pattern.0[(tile_size - 1) * tile_size + j] as usize;
                if prev == 3 || prev == 4 {
                    score += prev - 2;
                } else {
                    let col = pattern.0[i * tile_size + j] as usize;
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

// AHC001
// Problem: https://atcoder.jp/contests/ahc001/tasks/ahc001_a
// Submission: 

use proconio::input;
use rand::prelude::*;

const H: i32 = 10000;
const W: i32 = 10000;
const SEED: u128 = 0;
const TIME_LIMIT: f32 = 4.99;

struct Input {
    company_num: usize,
    ads_desired_point: Vec<(i32, i32)>,
    ads_desired_square: Vec<f32>,
}

fn input() -> Input {
    input! {
        n: usize,
        xyr: [(i32, i32, f32); n]
    }
    let company_num = n;
    let mut ads_desired_point = vec![];
    let mut ads_desired_square = vec![];
    for (x, y, r) in xyr {
        ads_desired_point.push((x, y));
        ads_desired_square.push(r);
    }
    Input {
        company_num,
        ads_desired_point,
        ads_desired_square,
    }
}

fn main() {
    let ads_info = input();
    let init_solution = calc_init_solution(&ads_info); // solution = ads_place
    let best_solution = annealing(&ads_info, init_solution); // annealing method
    print_solution(&best_solution);
}

fn calc_init_solution(ads_info: &Input) -> Vec<(i32, i32, i32, i32)> {
    let mut ads_place = vec![(0, 0, 0, 0); ads_info.company_num];
    for i in 0..ads_info.company_num {
        let (x, y) = ads_info.ads_desired_point[i];
        ads_place[i] = (x, y, x + 1, y + 1);
    }
    ads_place
}

fn annealing(
    ads_info: &Input,
    init_solution: Vec<(i32, i32, i32, i32)>
) -> Vec<(i32, i32, i32, i32)> {
    const START_TEMP: f32 = 2000.0;
    const END_TEMP: f32 = 50.0;

    let start_time = std::time::Instant::now();
    let mut solution = init_solution.clone();
    let mut best_solution = init_solution.clone();
    let mut score = calc_score(ads_info, &init_solution);
    let mut best_score = score;
    let mut rng = rand_pcg::Pcg64Mcg::new(SEED);
    let mut coordinate_choice = vec![0, 1, 2, 3]; 
    let mut iter_num = 0;

    'mainloop: loop {
        iter_num += 1;
        let diff_time = (std::time::Instant::now() - start_time).as_secs_f32();
        if diff_time > TIME_LIMIT {
            break;
        }
        let mut new_solution = solution.clone();
        let ad_index = rng.gen_range(0, ads_info.company_num);
        coordinate_choice.shuffle(&mut rng);
        let dl = if rng.gen::<f32>() > 0.2 { rng.gen_range(1, 10) } else { -rng.gen_range(1, 10) };
        for i in 0..5 {
            if i == 4 {
                continue 'mainloop
            }
            let c_index = coordinate_choice[i];
            match c_index {
                0 => {
                    new_solution[ad_index].0 -= dl;
                    if check_ad_size(ad_index, &new_solution) {
                        break;
                    } else {
                        new_solution[ad_index].0 += dl;
                    }
                }
                1 => {
                    new_solution[ad_index].1 -= dl;
                    if check_ad_size(ad_index, &new_solution) {
                        break;
                    } else {
                        new_solution[ad_index].1 += dl;
                    }
                }
                2 => {
                    new_solution[ad_index].2 += dl;
                    if check_ad_size(ad_index, &new_solution) {
                        break;
                    } else {
                        new_solution[ad_index].2 -= dl;
                    }
                }
                3 => {
                    new_solution[ad_index].3 += dl;
                    if check_ad_size(ad_index, &new_solution) {
                        break;
                    } else {
                        new_solution[ad_index].3 -= dl;
                    }
                }
                _ => unreachable!()
            }
        }
        
        let new_score = calc_score(ads_info, &new_solution);
        let temp = START_TEMP + (END_TEMP - START_TEMP) * diff_time / TIME_LIMIT;
        if f32::exp((new_score - score) / temp) > rng.gen() {
            score = new_score;
            solution = new_solution.clone();
        }
        if diff_time > 4.9 && new_score > best_score {
            best_score = new_score;
            best_solution = new_solution;
        }
    }
    eprintln!("Iteration = {}", iter_num);
    eprintln!("best score = {}", best_score);
    best_solution
}

fn check_ad_size(ad_index: usize, ads_place: &Vec<(i32, i32, i32, i32)>) -> bool {
    // (lx, ly) .___________.
    //          |           |
    //          .___________. (rx, ry)
    let (self_lx, self_ly, self_rx, self_ry) = ads_place[ad_index];
    if 0 > self_lx || 0 > self_ly || self_rx > H || self_ry > W {
        return false;
    }
    if self_lx >= self_rx || self_ly >= self_ry {
        return false;
    }

    for (other_ad_index, &(other_lx, other_ly, other_rx, other_ry)) in ads_place.iter().enumerate() {
        if ad_index == other_ad_index {
            continue;
        }
        ///////////////////////////////
        if other_lx <= self_lx && self_rx <= other_rx && self_ly <= other_ly && other_ly <= self_ry {
            return false;
        }
        if self_lx <= other_lx && other_rx <= self_rx && other_ly <= self_ly && self_ly <= other_ry {
            return false;
        }
        ///////////////////////////////
        // self ⊂ other
        // (left_x, left_y)
        if other_lx <= self_lx && self_lx <= other_rx && other_ly <= self_ly && self_ly <= other_ry {
            return false;
        }
        // (left_x, right_y)
        if other_lx <= self_lx && self_lx <= other_rx && other_ly <= self_ry && self_ry <= other_ry {
            return false;
        }
        // (right_x, left_y)
        if other_lx <= self_rx && self_rx <= other_rx && other_ly <= self_ly && self_ly <= other_ry {
            return false;
        }
        // (right_x, right_y)
        if other_lx <= self_rx && self_rx <= other_rx && other_ly <= self_ry && self_ry <= other_ry {
            return false;
        }

        ///////////////////////////////
        // other ⊂ self
        // (left_x, left_y)
        if self_lx <= other_lx && other_lx <= self_rx && self_ly <= other_ly && other_ly <= self_ry {
            return false;
        }
        // (left_x, right_y)
        if self_lx <= other_lx && other_lx <= self_rx && self_ly <= other_ry && other_ry <= self_ry {
            return false;
        }
        // (right_x, left_y)
        if self_lx <= other_rx && other_rx <= self_rx && self_ly <= other_ly && other_ly <= self_ry {
            return false;
        }
        // (right_x, right_y)
        if self_lx <= other_rx && other_rx <= self_rx && self_ly <= other_ry && other_ry <= self_ry {
            return false;
        }
    }
    return true;
}

fn calc_score(ads_info: &Input, ads_place: &Vec<(i32, i32, i32, i32)>) -> f32 {
    let mut score = 0.0;
    for i in 0..ads_info.company_num {
        let (x, y) = ads_info.ads_desired_point[i];
        let (lx, ly, rx, ry) = ads_place[i];
        if (lx <= x && x < rx) && (ly <= y && y < ry) {
            let desired_square = ads_info.ads_desired_square[i];
            let square = ((rx - lx) * (ry - ly)) as f32;
            let ratio = square / desired_square;
            score += ratio * (2.0 - ratio);
        }
    }
    score * 1e9 / (ads_info.company_num as f32)
}

fn print_solution(ads_place: &Vec<(i32, i32, i32, i32)>) {
    for (lx, ly, rx, ry) in ads_place {
        println!("{} {} {} {}", lx, ly, rx, ry);
    }
}

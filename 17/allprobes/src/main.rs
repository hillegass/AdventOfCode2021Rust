const STOP_X_MIN:i32 = 150;
const STOP_X_MAX:i32 = 171;
const STOP_Y_MAX:i32 = -70;
const STOP_Y_MIN:i32 = -129;

const MIN_XV:i32 = 13;

fn main() {

    // let min_v_x_0 = (1.0 + (1.0 + 8.0 * (STOP_X_MIN as f32)).sqrt()) / 2.0;
    // let mut min_v_x = min_v_x_0  as i32 - 1;

    let mut all_pairs = Vec::new();

    let mut abs_min_x = i32::max_value();
    let mut abs_max_x = i32::min_value();
    let mut abs_min_y = i32::max_value();
    let mut abs_max_y = i32::min_value();
    for trial_vy in STOP_Y_MIN..=130 as i32 {
        for trial_vx in MIN_XV..=STOP_X_MAX {
            let mut current_x = 0;
            let mut current_y = 0;
            let mut current_v_x = trial_vx;
            let mut current_v_y = trial_vy;
            while current_y >= STOP_Y_MIN && current_x <= STOP_X_MAX {
                current_x += current_v_x;
                current_y += current_v_y;
                current_v_y -= 1;
                if current_v_x > 0 {
                    current_v_x -= 1;
                }
 
                if current_y <= STOP_Y_MAX && current_y >= STOP_Y_MIN  && current_x >= STOP_X_MIN && current_x <= STOP_X_MAX {
                    all_pairs.push((trial_vx, trial_vy));
                    println!("Saving vx: {}, vy: {} ", trial_vx, trial_vy);
                    if trial_vx < abs_min_x {
                        abs_min_x = trial_vx;
                    }
                    if trial_vx > abs_max_x {
                        abs_max_x = trial_vx;
                    }
                    if trial_vy < abs_min_y {
                        abs_min_y = trial_vy;
                    }
                    if trial_vy > abs_max_y {
                        abs_max_y = trial_vy;
                    }
                    break
                }
            }
        }
    }
    // println!("{:?}", all_pairs);
    println!("X range: {} to {}", abs_min_x, abs_max_x);
    println!("Y range: {} to {}", abs_min_y, abs_max_y);    
    println!("{}", all_pairs.len());

}

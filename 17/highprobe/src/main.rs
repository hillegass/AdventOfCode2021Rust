const STOP_X_MIN:i32 = 150;
const STOP_X_MAX:i32 = 171;
const STOP_Y_MAX:i32 = -70;
const STOP_Y_MIN:i32 = -129;

fn main() {

    let v_x_0 = (1.0 + (1.0 + 8.0 * (STOP_X_MAX as f32)).sqrt()) / 2.0;
    println!("v_x_0: {}", v_x_0);
    let mut v_x = v_x_0 as i32 - 1 ;
    //let mut v_x = 18;
    println!("v_x: {}", v_x);

    let mut current_x = 0;
    while v_x > 0 {
        current_x += v_x;
        v_x -= 1;
    }
    let mut max_y = 0;
    for trial_vy in 100..700 as i32 {
        let mut current_y = 0;
        let mut current_v_y = trial_vy;
        let mut highest_point_of_flight = 0;
        while current_y >= STOP_Y_MIN {
        
            current_y += current_v_y;
            current_v_y -= 1;
            if current_v_y == 0 {
                highest_point_of_flight = current_y;
            }
            
            // Did we land it?
            if current_y <= STOP_Y_MAX && current_y >= STOP_Y_MIN {
                if highest_point_of_flight > max_y {
                    max_y = highest_point_of_flight;
                    println!("max_y: {} (start velocity: {}, position: {})", max_y, trial_vy, current_y);
                }
                break
            }
        }
    }
    
}

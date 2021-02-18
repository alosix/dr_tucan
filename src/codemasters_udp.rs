
struct mode_0 {
    total_time: i32,
    angular_velocity_x: f32,
    angular_velocity_z: f32,
    angular_velocity_y: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
    acceleration_x: f32,
    acceleration_z: f32,
    acceleration_y: f32,
    velocity_x; f32,
    velocity_z: f32,
    velocity_y: f32,
    position_x: f32,
    position_z: f32,
    position_y: f32
    to_ca: [u8,4] // Should just contain ToCA
}

struct mode_1 {
    total_time: f32, 
    lap_time: f32, 
    lap_distance: f32, 
    total_distance: f32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    speed: f32, // Its not the speed, its suddenly becoming stationary that gets you
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    left_dir_x: f32,
    left_dir_y: f32,
    left_dir_z: f32,
    forward_dir_x: f32,
    forward_dir_y: f32,
    forward_dir_z: f32,
    suspension_position_bl: f32, // scale 1000.0
    suspension_position_br: f32, // scale 1000.0
    suspension_position_fl: f32, // scale 1000.0
    suspension_position_fr: f32, // scale 1000.0
    suspension_velocity_bl: f32, // scale 1000.0
    suspension_velocity_br: f32, // scale 1000.0
    suspension_velocity_fl: f32, // scale 1000.0
    suspension_velocity_fr: f32, // scale 1000.0
    wheel_patch_speed_bl: f32, 
    wheel_patch_speed_br: f32,
    wheel_patch_speed_fl: f32,
    wheel_patch_speed_fr: f32,
    throttle_input: f32,
    steering_input: f32,
    clutch_input: f32,
    gear: f32,
    gforce_lateral: f32,
    gforce_logitudinal: f32,
    lap: f32,
    engine_rate: f32
}

struct mode_2 {
    total_time: f32, 
    lap_time: f32, 
    lap_distance: f32, 
    total_distance: f32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    speed: f32, // Its not the speed, its suddenly becoming stationary that gets you
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    left_dir_x: f32,
    left_dir_y: f32,
    left_dir_z: f32,
    forward_dir_x: f32,
    forward_dir_y: f32,
    forward_dir_z: f32,
    suspension_position_bl: f32, // scale 1000.0
    suspension_position_br: f32, // scale 1000.0
    suspension_position_fl: f32, // scale 1000.0
    suspension_position_fr: f32, // scale 1000.0
    suspension_velocity_bl: f32, // scale 1000.0
    suspension_velocity_br: f32, // scale 1000.0
    suspension_velocity_fl: f32, // scale 1000.0
    suspension_velocity_fr: f32, // scale 1000.0
    wheel_patch_speed_bl: f32, 
    wheel_patch_speed_br: f32,
    wheel_patch_speed_fl: f32,
    wheel_patch_speed_fr: f32,
    throttle_input: f32,
    steering_input: f32,
    clutch_input: f32,
    gear: f32,
    gforce_lateral: f32,
    gforce_logitudinal: f32,
    lap: f32,
    engine_rate: f32,
    native_sli_support: f32,
    race_position: f32,
    kers_level: f32,
    kers_level_max: f32,
    drs: f32,
    traction_control: f32,
    abs: f32,
    fuel_in_tank: f32,
    fuel_capacity: f32,
    in_pits: f32,
    race_sector: f32,
    sector_time_1: f32,
    sector_time_2: f32,
    brake_temp_bl: f32,
    brake_temp_br: f32,
    brake_temp_fl: f32,
    brake_temp_fr: f32,
    tyre_pressure_b1: f32,
    tyre_pressure_br: f32,
    tyre_pressure_f1: f32,
    tyre_pressure_fr: f32,
    laps_completed: f32,
    total_laps: f32,
    track_length: f32,
    last_lap_time: f32
}

//
// Info from :https://www.scribd.com/document/350826037/UDP-output-setup		
// Comments mostly based on DR series
struct mode_3 {
    total_time: f32, // Total time (not reset after stage restart)
    lap_time: f32,  // Current Lap/Stage Time (starts on Go!)
    lap_distance: f32,  // Current Lap/Stage Distance (meters)
    total_distance: f32, 
    position_x: f32,
    position_y: f32,
    position_z: f32,
    speed: f32, // Its not the speed, its suddenly becoming stationary that gets you (m/s)
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    left_dir_x: f32,
    left_dir_y: f32,
    left_dir_z: f32,
    forward_dir_x: f32,
    forward_dir_y: f32,
    forward_dir_z: f32,
    suspension_position_bl: f32, // scale 1000.0
    suspension_position_br: f32, // scale 1000.0
    suspension_position_fl: f32, // scale 1000.0
    suspension_position_fr: f32, // scale 1000.0
    suspension_velocity_bl: f32, // scale 1000.0
    suspension_velocity_br: f32, // scale 1000.0
    suspension_velocity_fl: f32, // scale 1000.0
    suspension_velocity_fr: f32, // scale 1000.0
    wheel_patch_speed_bl: f32, 
    wheel_patch_speed_br: f32,
    wheel_patch_speed_fl: f32,
    wheel_patch_speed_fr: f32,
    throttle_input: f32,
    steering_input: f32,
    clutch_input: f32,
    gear: f32, // 1  =1 ,etc.. except 10 is reverse
    gforce_lateral: f32,
    gforce_logitudinal: f32,
    lap: f32,
    engine_rate: f32,
    native_sli_support: f32,
    race_position: f32,
    kers_level: f32,
    kers_level_max: f32,
    drs: f32,
    traction_control: f32,
    abs: f32,
    fuel_in_tank: f32,
    fuel_capacity: f32,
    in_pits: f32,
    race_sector: f32,
    sector_time_1: f32,
    sector_time_2: f32,
    brake_temp_bl: f32, // Temp in C
    brake_temp_br: f32, // Temp in C
    brake_temp_fl: f32, // Temp in C
    brake_temp_fr: f32, // Temp in C
    tyre_pressure_b1: f32,
    tyre_pressure_br: f32,
    tyre_pressure_f1: f32,
    tyre_pressure_fr: f32,
    laps_completed: f32,
    total_laps: f32,
    track_length: f32,
    last_lap_time: f32,
    max_rpm: f32,
    idle_rpm: f32,
    max_gears: f32,
    session_type: f32,
    drs_allowed: f32,
    track_number; f32, // used to id the stage
    vehicle_fia_flags: f32
}
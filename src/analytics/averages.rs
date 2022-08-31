pub fn moving_average(data: &Vec<f32>, length: u32) -> f32 {
    let size = length as usize;
    let elements = data.as_slice()[..size].to_vec();
    let result: f32 = elements.into_iter().sum();

    result / length as f32
}

// TODO: double check this calculation
pub fn exponential_moving_average(data: &Vec<f32>, length: u32) -> f32 {
    let size = length as usize;
    let elements: Vec<f32> = data.as_slice()[..size].to_vec().into_iter().rev().collect();

    let smoothing = 2.0 / (length as f32 + 1.0);

    let sma = moving_average(data, length);
    let recent_element = elements.first().unwrap();

    (recent_element * smoothing) + sma * (1.0 - smoothing)
}

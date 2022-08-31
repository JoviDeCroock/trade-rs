pub fn get_percentage_difference(first: &f32, second: &f32) -> f32 {
    if first == second {
        return 0.0;
    }

    ((first / second) * 100.0) - 100.0
}

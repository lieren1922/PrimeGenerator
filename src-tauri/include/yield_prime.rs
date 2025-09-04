fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[tauri::command]
pub async fn yield_prime(nums: u64) -> Vec<u64> {
    (1..=nums)
        .filter(|&num| is_prime(num))
        .collect::<Vec<u64>>()
}

#[tauri::command]
pub async fn yield_prime_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end)
        .filter(|&num| is_prime(num))
        .collect::<Vec<u64>>()
}

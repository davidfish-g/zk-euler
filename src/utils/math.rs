pub fn factorial(n: u32) -> u64 {
    (2..=n as u64).product()
}

pub fn sum_proper_divisors(num: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=num / 2 {
        if num % i == 0 {
            sum += i;
        }
    }
    sum
}

pub fn num_divisors(num: i32) -> i32 {
    let sqrt = (num as f64).sqrt() as i32;
    let mut divisors = 0;
    for i in 1..=num / sqrt {
        if num % i == 0 {
            divisors += 1;
        }
    }
    for i in 2..sqrt {
        if num % i == 0 {
            divisors += 1;
        }
    }
    divisors + 1
}

pub fn triangle_path_sum(triangle: &mut [Vec<i32>]) -> i32 {
    let len = triangle.len();
    for i in (0..len - 1).rev() {
        for j in 0..=i {
            if triangle[i + 1][j] > triangle[i + 1][j + 1] {
                triangle[i][j] += triangle[i + 1][j];
            } else {
                triangle[i][j] += triangle[i + 1][j + 1];
            }
        }
    }
    triangle[0][0]
}

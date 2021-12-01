#[derive(Debug, PartialOrd, PartialEq)]
pub enum Gradient {
    Increased,
    Decreased,
    NoDifference
}

impl std::fmt::Display for Gradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gradient::Increased =>  write!(f, "Increased"),
            Gradient::Decreased =>  write!(f, "Decreased"),
            _ => write!(f, "No difference")
        }
    }
}

pub fn read_lines(reads: &str) -> Vec<u32> {
    let mut measurements: Vec<u32> = vec![];
    for line in reads.lines() {
        println!("{}", line);
        measurements.push(line.parse().expect("Could not read number from line."));
    };

    measurements
}

pub fn count_gradient_type(gradients: &[Gradient], gradient_type: Gradient) -> usize {
    let mut count = 0;

    for gradient in gradients {
        if *gradient == gradient_type {
            count += 1;
        }
    }

    count
}

fn depth_difference(previous: u32, next: u32) -> Gradient {
    match previous as i64 - next as i64 {
        value if value > 0 => Gradient::Decreased,
        value if value < 0 => Gradient::Increased,
        _ => Gradient::NoDifference
    }
}

pub fn calculate_windowed_measurements(measurements: &[u32]) -> Vec<u32> {
    let count_windows = measurements.len() - 2;
    let mut windowed_measurements = Vec::with_capacity(count_windows);

    for index in 0..count_windows {
        windowed_measurements.push(measurements[index] + measurements[index + 1] + measurements[index + 2]);
    }

    windowed_measurements
}

pub fn calculate_depth_gradient(measurements: &[u32]) -> Vec<Gradient> {
    let mut differences = Vec::with_capacity(measurements.len());

    for index in 0..measurements.len() {
        if index == 0 {
            differences.push(Gradient::NoDifference);
            continue;
        } else {
            differences.push(depth_difference(measurements[index - 1], measurements[index]));
        }
    }

    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        let measurements = read_lines(input);

        assert_eq!(measurements, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    }

    #[test]
    fn test_count_gradient_increased() {
        let gradients = vec![
            Gradient::NoDifference,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
        ];

        let count = count_gradient_type(&gradients, Gradient::Increased);

        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_gradient_decreased() {
        let gradients = vec![
            Gradient::NoDifference,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
        ];

        let count = count_gradient_type(&gradients, Gradient::Decreased);

        assert_eq!(count, 2);
    }

    #[test]
    fn test_count_gradient_no_difference() {
        let gradients = vec![
            Gradient::NoDifference,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
        ];

        let count = count_gradient_type(&gradients, Gradient::NoDifference);

        assert_eq!(count, 1);
    }

    #[test]
    fn test_increased_depth() {
        let first = 200;
        let second = 220;

        let result = depth_difference(first, second);

        assert_eq!(result, Gradient::Increased);
    }

    #[test]
    fn test_decreased_depth() {
        let first = 220;
        let second = 200;

        let result = depth_difference(first, second);

        assert_eq!(result, Gradient::Decreased);
    }

    #[test]
    fn test_no_difference_in_depth() {
        let first = 200;
        let second = 200;

        let result = depth_difference(first, second);

        assert_eq!(result, Gradient::NoDifference);
    }

    #[test]
    fn test_calculate_windowed_depth_measurements() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let windowed_measurements = vec![607, 618, 618, 617, 647, 716, 769, 792];

        let result = calculate_windowed_measurements(&measurements);

        assert_eq!(result, windowed_measurements);
    }

    #[test]
    fn test_calculate_depth_gradients() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = calculate_depth_gradient(&measurements);

        assert_eq!(result, vec![
            Gradient::NoDifference,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Increased,
            Gradient::Decreased,
            Gradient::Increased,
        ]);
    }
}
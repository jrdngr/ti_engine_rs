use std::f64::consts::PI;

pub fn to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}

pub fn to_degrees(radians: f64) -> f64 {
    (radians * 180.0) / PI
}

// This function does the equivalent of a try/catch block. Rust uses Result types
// instead of exceptions for error handling. A result looks like this:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// This means that anything returning a Result can either return a T or an E.
// You figure out which one it is by using a "match" statement.
pub fn string_to_int(string: &str) -> i32 {
    match string.parse::<i32>() {
        Ok(num) => num,
        Err(_)  => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_radians() {
        assert_eq!(to_radians(0.0), 0.0);
        assert_eq!(to_radians(14.323945045471191), 0.2500000029182011);
        assert_eq!(to_radians(28.647890090942383), 0.5000000058364023);
        assert_eq!(to_radians(42.971836090087891), 0.7500000253993603);
        assert_eq!(to_radians(57.2957801818847661), 1.0000000116728045);
    }

    #[test]
    fn test_to_degrees() {
        assert_eq!(to_degrees(0.0), 0.0);
        assert_eq!(to_degrees(0.2500000029182011), 14.323945045471188);
        assert_eq!(to_degrees(0.5000000058364023), 28.647890090942376);
        assert_eq!(to_degrees(0.7500000253993603), 42.971836090087891);
        assert_eq!(to_degrees(1.0), 57.29577951308232);    
    }

    #[test]
    fn test_string_to_int() {
        let one = string_to_int("1");
        let two = string_to_int("2");
        let three = string_to_int("3");
    
        assert_eq!(one, 1);
        assert_eq!(two, 2);
        assert_eq!(three, 3);
    }
}

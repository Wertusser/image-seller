pub fn euclidean_dist(point_1: (i32, i32), point_2: (i32, i32)) -> f32 {
    let (x1, y1) = point_1;
    let (x2, y2) = point_2;
    let squared_result = ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32;
    squared_result.sqrt()
}

pub fn avg(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn pearson(x: &[i32], y: &[i32]) -> Result<f32, String> {
    let n: usize = x.len();
    if n != y.len() {
        Err(format!(
            "Length of vector {:?} =! length of vector {:?} !",
            x, y
        ))
    } else if n == 0 {
        Err(String::from("There are empty vectors!"))
    } else {
        let avg_x = avg(x);
        let avg_y = avg(y);
        let mut diffprod: f32 = 0.0;
        let mut xdiff2: f32 = 0.0;
        let mut ydiff2: f32 = 0.0;
        for i in 0..x.len() {
            let xdiff = x[i] as f32 - avg_x;
            let ydiff = y[i] as f32 - avg_y;
            diffprod += xdiff * ydiff;
            xdiff2 += xdiff * xdiff;
            ydiff2 += ydiff * ydiff;
        }
        let result = diffprod / ((xdiff2 * ydiff2).sqrt());
        Ok(result)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_euclidian_dist() {
        let dist_1 = euclidean_dist((1, 1), (1, 1));
        let dist_2 = euclidean_dist((1, 1), (10, 10));
        let dist_3 = euclidean_dist((10, 11), (23, -4));

        assert_eq!(dist_1, 0.0);
        assert_eq!(dist_2, 12.727922);
        assert_eq!(dist_3, 19.849434);
    }

    #[test]
    fn test_pearson_corr() {
        let corrs = vec![
            pearson(&[1, 2, 3], &[1, 2, 3, 4]),
            pearson(&[1, 2, 3, 4], &[1, 2, 3]),
            pearson(&[], &[]),
            pearson(&[1, 2, 3], &[3, 4, 5]),
        ];
        let mut result = Vec::new();
        let mut errs = Vec::new();
        for corr in corrs {
            match corr {
                Ok(n) => result.push(n),
                Err(e) => errs.push(e),
            }
        }
        assert_eq!(vec![1.0], result);
        assert_eq!(
            vec![
                "Length of vector [1, 2, 3] =! length of vector [1, 2, 3, 4] !",
                "Length of vector [1, 2, 3, 4] =! length of vector [1, 2, 3] !",
                "There are empty vectors!",
            ],
            errs
        )
    }
}

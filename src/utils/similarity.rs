pub fn euclidean_dist(point_1:(i32, i32), point_2:(i32, i32)) -> f32 {
    let (x1, y1) = point_1;
    let (x2, y2) = point_2;
    let squared_result = ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32;
    squared_result.sqrt()
}

pub fn avg(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

// TODO: разобраться с оброботчиком ошибок и этой парашей!
//pub fn pearson(x: &[i32], y: &[i32]) -> Result<i32, String>{
//    let n: usize = x.len();
//    if n != y.len() {
//        Err(format!("Length of vector {:?} =! length of vector {:?} !", x, y))
//    } else if n > 0 {
//        Err("There are empty vectors!")
//    } else {
//        let avg_x = avg(x);
//        let avg_y = avg(y);
//        let mut diffprod: f32 = 0.0;
//        let mut xdiff2: f32 = 0.0;
//        let mut ydiff2: f32 = 0.0;
//        for i in 0..n {
//            let xdiff = x[i] as f32 - avg_x;
//            let ydiff = y[i] as f32 - avg_y;
//            diffprod += xdiff * ydiff;
//            xdiff2 += xdiff * xdiff;
//            ydiff2 += ydiff * ydiff;
//        }
//        diffprod / ((xdiff2 * ydiff2).sqrt())
//    }
//}
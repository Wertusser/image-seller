#[cfg(test)]
mod tests {
    extern crate kawasaki;

    use tests::kawasaki::utils::similarity;

    #[test]
    fn test_euclidian_dist() {
        let dist_1 = similarity::euclidean_dist((1, 1), (1, 1));
        let dist_2 = similarity::euclidean_dist((1, 1), (10, 10));
        let dist_3 = similarity::euclidean_dist((10, 11), (23, -4));

        assert_eq!(dist_1, 0.0);
        assert_eq!(dist_2, 12.727922);
        assert_eq!(dist_3, 19.849434);
    }

    fn test_pearson_corr() {}
}
#[cfg(test)]
mod test {
    use crate::converter::weight_converter::calculate_weight_on_mars;

    #[test]
    fn check_calculate_weight_on_mars_success() {
        assert_eq!(calculate_weight_on_mars(100.0), 37.828747);
    }
}
#[cfg(test)]
mod tests {
    use test_ignore_if::ignore_if;

    #[ignore_if(env_set = "CI", enabled = true)]
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[ignore_if(env_set = "CI", enabled = false)]
    #[test]
    fn it_works_bis() {
        assert_eq!(2 + 2, 4);
    }

    #[ignore_if(env_set = "CI")]
    #[test]
    fn it_works_ter() {
        assert_eq!(2 + 2, 4);
    }
}

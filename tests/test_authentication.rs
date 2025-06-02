mod test {
	use actix_authentication_middleware::User;
	use log::LevelFilter;

	#[tokio::test]
    async fn login_with_username_password() {
		let _ = pretty_env_logger::env_logger::builder()
			.is_test(true)
			.format_timestamp(None)
			.filter_level(LevelFilter::Trace)
			.try_init();

        let user = User::authenticate_user("mardens", "mardens", "Mardens Actix Auth Library")
            .await
            .expect("Failed to authenticate user")
            .expect("User not found");
        assert_eq!(
            user,
            r#"eyJ1c2VybmFtZSI6Im1hcmRlbnMiLCJhZG1pbiI6ZmFsc2UsInRva2VuIjoiYzVlMjNjYmNkZjVkNzk3MWQ3ZmNkZWRjOWI3OWE1NjQwOWI1YzI1MWZiYzM5OGU2NmVlY2JkMzU0M2E2ODRkYyJ9"#
        )
    }

    #[tokio::test]
    async fn login_with_token() {
        let _ = pretty_env_logger::env_logger::builder()
            .is_test(true)
            .format_timestamp(None)
            .filter_level(LevelFilter::Trace)
            .try_init();
        const TOKEN: &str = "eyJ1c2VybmFtZSI6Im1hcmRlbnMiLCJhZG1pbiI6ZmFsc2UsInRva2VuIjoiYzVlMjNjYmNkZjVkNzk3MWQ3ZmNkZWRjOWI3OWE1NjQwOWI1YzI1MWZiYzM5OGU2NmVlY2JkMzU0M2E2ODRkYyJ9";
        const USER_AGENT: &str = "Mardens Actix Auth Library";
        User::authenticate_user_with_token(TOKEN, USER_AGENT)
            .await
            .expect("Failed to authenticate user");
    }
}

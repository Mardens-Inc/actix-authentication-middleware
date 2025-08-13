mod test {
	use actix_authentication_middleware::User;
	use log::LevelFilter;
	const TOKEN: &str = "eyJ1c2VybmFtZSI6Im1hcmRlbnMiLCJhZG1pbiI6ZmFsc2UsInRva2VuIjoiYTQ3MzM0NzNjMGU4MjA3YTdkNGZjNDIxMjRmMmY5ODJiY2JjZjEzMWQ4OGY2MTI5Yjk5YTliNjMwZGVlMzUwYyJ9";

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
        assert_eq!(user, TOKEN)
    }

    #[tokio::test]
    async fn login_with_token() {
        let _ = pretty_env_logger::env_logger::builder()
            .is_test(true)
            .format_timestamp(None)
            .filter_level(LevelFilter::Trace)
            .try_init();
        const USER_AGENT: &str = "Mardens Actix Auth Library";
        User::authenticate_user_with_token(TOKEN, USER_AGENT)
            .await
            .expect("Failed to authenticate user");
    }
    #[tokio::test]
    async fn get_user_by_token() {
        let _ = pretty_env_logger::env_logger::builder()
            .is_test(true)
            .format_timestamp(None)
            .filter_level(LevelFilter::Trace)
            .try_init();
        let user = User::get_user_from_token(TOKEN)
            .await
            .expect("Failed to get user")
            .expect("User not found");
        assert_eq!(user.username, "mardens")
    }
}

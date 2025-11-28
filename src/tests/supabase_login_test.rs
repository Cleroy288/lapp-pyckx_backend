use crate::config::Config;
use crate::infrastructure::SupabaseClient;

#[tokio::test]
async fn test_supabase_login_real() {
    // load real config
    let cfg = Config::from_env();
    let supabase = SupabaseClient::new(&cfg);

    // test with a real user
    let email = "[email]";
    let password = "[password]";

    let result = supabase.login(email, password).await;

    match result {
        Ok(response) => {
            println!("=== SUPABASE RAW RESPONSE ===");
            println!("{:#?}", response);
            println!("==============================");

            // we just check the response is not empty
            assert!(!response.access_token.is_empty());
        }
        Err(err) => panic!("Supabase login failed: {:?}", err),
    }
}

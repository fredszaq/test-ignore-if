pub fn enable_ignore_if_env_set_for(env_name: &str) {
    println!("cargo:rustc-env=TEST_IGNORE_IF_ENV_SET_ENABLED_FOR_{}=1", env_name);
    println!("cargo:rerun-if-env-changed={}", env_name);
}

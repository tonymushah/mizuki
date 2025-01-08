const COMMANDS: &[&str] = &["graphql", "subscriptions"];

pub fn build() {
  tauri_plugin::Builder::new(COMMANDS).build();
}

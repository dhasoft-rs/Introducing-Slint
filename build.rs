
fn main() {

  // https://docs.slint.dev/latest/docs/slint/src/advanced/style
  // Pour compiler en utilisant un thème
  // Thèmes possibles : material, fluent, cupertino, cosmic, qt (si installé), native
  // suivi de -dark pour la version dark ...
  let config = slint_build::CompilerConfiguration::new().with_style("material-dark".into());
  slint_build::compile_with_config("ui/app-window.slint", config).unwrap();

  // Pour compiler sans thème (mais le native semble ne pas être bien détecté)
  //slint_build::compile("ui/app-window.slint").expect("Slint build failed");

}



#[test]
fn ready() {
    println!("it works!")
}

use valkyrie_interpreter::ValkyrieVM;

const WRONG: &str = r#"namespace test;
let a = 1⁏
let mut ma = 1⁏
"#;

#[tokio::test]
async fn debug_wrong() {
    let mut vm = ValkyrieVM::default();
    let values = vm.execute_script(WRONG).await;
    println!("{:#?}", values);
}

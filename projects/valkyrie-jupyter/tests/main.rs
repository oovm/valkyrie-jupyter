#[test]
fn ready() {
    println!("it works!")
}

use valkyrie_interpreter::ValkyrieVM;

const WRONG: &str = r#"namespace test;
// empty table
()⁏
// list like table
(1, 2, 3)⁏
// dict like table
(
    a: 1, 
    b: 2,
)⁏
// mix style table
(
    1, 2, 3,
    a: 1,
    b: 2,
)⁏
"#;

#[tokio::test]
async fn debug_wrong() {
    let mut vm = ValkyrieVM::default();
    let values = vm.execute_script(WRONG).await;
    println!("{:#?}", values);
}

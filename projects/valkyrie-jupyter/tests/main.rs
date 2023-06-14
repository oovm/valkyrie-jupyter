#[test]
fn ready() {
    println!("it works!")
}

use valkyrie_interpreter::ValkyrieVM;

const SWITCH: &str = r#"
switch {
    when true:
        1,
    else:
        2,
}
switch {
    when false:
        1,
    else:
        2,
}
"#;

#[tokio::test]
async fn running() {
    let mut vm = ValkyrieVM::default();
    let values = vm.execute_script(SWITCH).await;
    println!("{:#?}", values);
}

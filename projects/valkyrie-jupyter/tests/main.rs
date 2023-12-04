#[test]
fn ready() {
    println!("it works!")
}

use valkyrie_interpreter::{ValkyrieVM, ValkyrieValue};

const WRONG: &str = r#"namespace test;
"3";
"#;

#[tokio::test]
async fn debug_wrong() {
    let mut vm = ValkyrieVM::default();
    let file = vm.load_snippet(WRONG, "wrong.vk");

    for i in vm.execute_script(file).await {
        match i {
            Ok(o) => {
                println!("{:#?}", o);
            }
            Err(e) => {
                e.as_report().eprint(vm.as_ref()).ok();
            }
        }
    }
}

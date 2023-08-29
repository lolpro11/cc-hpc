use evalexpr::*;
fn main() {
    for i in 1..512 {
        let mut context = HashMapContext::new();
        context.set_value("i".into(), i.into()).expect("Failed to export value");
        let n = eval_with_context_mut("(2^(2*i) - 1)/3", &mut context);
        println!("{}", n.unwrap())
    }
}

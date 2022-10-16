use criterion::{criterion_group, criterion_main, Criterion};
use huckleberry_lib::{env::Env, evaluator::eval};

fn fibonacci_recursion(c: &mut Criterion) {
    let env = Env::with_core_module().into_ref();
    // Preparse the code to restict performance measurement to the interpreter.
    eval(
        "
        (defn fib [n] 
            (if (lt n 2) 1 
                (+ (fib (- n 1)) 
                   (fib (- n 2)))))",
        env.clone_ref(),
    )
    .unwrap();

    c.bench_function("fib_recursion_20", |b| {
        b.iter(|| eval("(fib 20)", env.clone_ref()))
    });
}

criterion_group!(benches, fibonacci_recursion);
criterion_main!(benches);

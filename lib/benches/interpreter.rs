use criterion::{criterion_group, criterion_main, Criterion};
use huckleberry_lib::{env::Env, evaluator::eval_exprs, parser::parse};

fn fibonacci_recursion(c: &mut Criterion) {
    let env = Env::with_core_module().into_ref();
    // Preparse the code to restict performance measurement to the interpreter.
    let exprs = parse(
        "
        (def fib (fn [n] 
            (if (< n 2) 1 
                (+ (fib (- n 1)) (fib (- n 2))))))
        (fib 20)",
    )
    .unwrap();

    c.bench_function("fib_recursion_20", |b| {
        b.iter(|| eval_exprs(&exprs, env.clone_ref()))
    });
}

criterion_group!(benches, fibonacci_recursion);
criterion_main!(benches);

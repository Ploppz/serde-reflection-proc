//! Compare the format that is generated with `serde-reflection-proc`, to that of
//! the `serde-refelection` `Tracer`.

use serde::Deserialize;
use serde_reflection::{Samples, Tracer, TracerConfig, Registry};
use serde_reflection_proc::Reflection;
use serde_reflection_proc_derive::Reflection;
use std::collections::HashMap;
// use serde_reflection_derive::{Reflection};

#[derive(Deserialize, Reflection)]
struct Foo {
    pub bar: Bar,
    pub x: i32,
    pub y: HashMap<String, u16>,
}

#[derive(Deserialize, Reflection)]
struct FooFlatten {
    #[serde(flatten)]
    pub bar: Bar,
    pub x: i32,
}

#[derive(Deserialize, Reflection)]
struct Bar {
    pub ty: Choice,
    pub result: f32,
}

#[derive(Deserialize, Reflection)]
enum Choice {
    A,
    B,
    C,
}

struct Abc;

fn main() {
    // Start the tracing session.
    { // WITH SERDE
        let mut tracer = Tracer::new(TracerConfig::default());
        let mut samples = Samples::new();

        tracer.trace_type::<Foo>(&samples).unwrap();
        tracer.trace_type::<Bar>(&samples).unwrap();
        tracer.trace_type::<Choice>(&samples).unwrap();

        let registry = tracer.registry().unwrap();
        println!("With serde:\n{:#?}", registry);
    }

    { // WITH PROC MACRO
        let mut registry = Registry::default();
        Foo::register(&mut registry).unwrap();
        FooFlatten::register(&mut registry).unwrap();
        Bar::register(&mut registry).unwrap();
        Choice::register(&mut registry).unwrap();

        println!("With proc macro:\n{:#?}", registry);
    }
}

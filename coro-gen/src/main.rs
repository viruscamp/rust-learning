#![feature(coroutines)]
#![feature(gen_blocks)]

fn main() {
    println!("Hello, world!");
    gen_movable();
    gen_immovable();
}

fn gen_movable() {
    println!("gen block");
    let gen1 = gen {
        yield 1;
        yield 2;
    };
    for x in gen1 {
        println!("{x}");
    }
    
    println!("gen-iter");
    let gen1 = gen_iter::gen_iter!({
        yield 1;
        yield 2;
    });
    for x in gen1 {
        println!("{x}");
    }
}

fn gen_immovable() {
    println!("gen block immovable: impossible");
    
    println!("gen-iter immovable");
    let gen1 = gen_iter::gen_iter!(static {
        let arr = [1,2];
        let rarr = &arr;
        for x in rarr {
            yield *x;
        }
    });
    for x in gen1 {
        println!("{x}");
    }
}

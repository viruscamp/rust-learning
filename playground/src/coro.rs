#[cfg(test)]
mod test {
    use std::{ops::{Coroutine, CoroutineState}, pin::{pin, Pin}};

    fn self_ref_coro(arg: (i32, i32)) -> impl Coroutine<Yield = i32, Return = ()> {
        #[coroutine] static move || {
            let v1 = [arg.0, arg.1, 3];
            let v = &v1;
            for i in 0..v.len() {
                yield v[i];
            }
        }
    }

    #[test]
    fn test_self_ref_coro() {
        let x = self_ref_coro((1,2));
        let bx = x;

        let mut c1 = pin!(self_ref_coro((1,2)));
        assert_eq!(c1.as_mut().resume(()), CoroutineState::Yielded(1));

        let mut c2 = Pin::<&mut _> { __pointer: &mut { self_ref_coro((11,12)) } };
        assert_eq!(c2.as_mut().resume(()), CoroutineState::Yielded(11));

        assert_eq!(c1.as_mut().resume(()), CoroutineState::Yielded(2));
        assert_eq!(c1.as_mut().resume(()), CoroutineState::Yielded(3));

        assert_eq!(c2.as_mut().resume(()), CoroutineState::Yielded(12));
    }

    #[test]
    fn movable() {
        // immovable
        let c1 = #[coroutine] static || {
            let x = 4;
            let y = &x;
            yield;
            dbg!(y);
        };
    }
}

# 泛型参数可变性

定义示例类型继承树: 
```java
class Animal {}
class Cat extends Animal {}
class Dog extends Animal {}
class CorgiDog extends Dog {}
```
那么`Dog`是`Animal`的子类型, 或者说实际类型`Dog`可以替换类型`Animal`.

`C<T>`是一个泛型类或接口.

1. Covariance 协变  
  如果某个泛型参数类型可以由其子类型替换, 那么此泛型类支持协变的.  
  如果`C<Dog>`是`C<Animal>`的子类型, 那么`C<T>`对于`T`协变.  
  Java写法: 实际类型`List<Dog>`可以替换类型`List<? extends Animal>`.  
  C#写法: `interface IEnumerable<out T>`, 实际类型`IEnumerable<Dog>`可以替换类型`IEnumerable<Animal>`.  
  特别注意: C#和Java的数组都是协变的`Animal[] al = new Dog[1];`, 协变后写入可能导致运行时错误.  
```java
// 想做的事
void checkAnimals(List<Animal> animals) {
    for(Animal a : animals) {
        a.check();
    }
}
checkAnimals(new ArayyList<Dog>()); // 类型错误

// 正确做法 泛型协变
void checkAnimals(List<? extends Animal> animals) {
    for(Animal a : animals) {
        a.check();
    }
    //animals.add(new Cat()); // 无法编译
}
checkAnimals(new ArayyList<Dog>()); // 类型正确

// 想做的事 泛型返回值
List<Animal> getList() {
    return new ArrayList<Dog>(); // 编译错误
}
Animal a = getList().get(0);

// 正确做法 泛型协变返回值
List<? extends Animal> getList() {
    return new ArrayList<Dog>();
}
Animal a = getList().get(0);

// 数组协变
Animal[] al = new Dog[1]; // 编译通过
al[0] = new Cat(); // 运行时错误
```

2. Contravariance 逆变  
  如果某个泛型参数类型可以由其父类型替换, 那么此泛型类支持逆变的.  
  如果`C<Animal>`是`C<Dog>`的子类型, 那么`C<T>`对于`T`逆变.  
  Java写法: 实际类型`List<Animal>`可以替换类型`List<? super Dog>`.  
  C#写法: `interface IComparer<in T>`, 实际类型`IComparer<Animal>`可以替换类型`IComparer<Dog>`.  
  C#不支持集合类型的逆变.  
```java
// 想做的事
void addDog(List<Animal> al) {
    al.add(new Dog());
}
addDog(new ArrayList<Dog>()); // 编译错误 因为`List<Animal>`不支持逆变

// 正确做法 泛型逆变
void addDog(List<? super Dog> al) {
    al.add(new Dog());
}
addDog(new ArrayList<Dog>()); // OK 逆变
addDog(new ArrayList<Animal>()); // OK
```

3. Invariance 抗变  
如果`C<Animal>`与`C<Dog>`不能互相替换, 那么`C<T>`对于`T`抗变, 或称不变.
```java
void useList(List<Dog> cl) {
    Dog d = cl.get(0); // List<Animal> 不能用
    cl.add(new Dog()); // List<CorgiDog> 不能用
}
useList(new ArrayList<Animal>()); // 错误
useList(new ArrayList<CorgiDog>()); // 错误
```

4. 函数类型的协变逆变  
我们希望`fn(Animal) -> CorgiDog`可以替代`fn(Cat) -> Dog`,
那么`fn(T) -> R`需要对于`T`逆变,对于`R`协变.
```java
// Java 代码
@Functional
public interface Function<T, R> {
    R apply(T t);
}

void use(Function<? super Cat, ? extends Dog> func) {
    Cat c = new Cat();
    Dog d = func.apply(c);
}

Dog func1(Cat c) {
    return new Dog();
}

CorgiDog func2(Animal a) {
    return new CorgiDog();
}

use(this::func1);
use(this::func2);
```
```c#
// C# 代码
public delegate TResult Func<in T, out TResult>(T arg);

void use(Func<Cat, Dog> func) {
    Cat c = new Cat();
    Dog d = func(c);
}

Dog func1(Cat c) {
    return new Dog();
}

CorgiDog func2(Animal a) {
    return new CorgiDog();
}

use(func1);
use(func2);
```

5. Rust  
Rust 没有 struct 继承, trait 继承不能用于类型转换.
Rust 的子类型关系只出现在生存期上.
```rust
fn lifetime<'big: 'small, 'small>(a: &'small i32, b: &'big i32) {}
```
意味着 `'big`是一个较长的生存期, 它能完全覆盖`'small`这个较短的生存期,
那么需要一个`&'small i32`的地方`&'big i32`是能够满足的,
所以`&'big i32`是`&'small i32`的子类型.

静态生存期`&'static T`是任意生存期`&'x T`的子类型.

Rust 泛型类型的可变性不是由语法定义,而是固定的几个基础类型的可变性表,
然后组合类型 `struct` `enum` 和 `union` 根据其包含域类型的可变性确定, 
域类型有多种可变性时, 组合类型为不变.

`Cell<T>` 包含 `std::cell::UnsafeCell<T>` 其对`T`不变.  
`Vec<T>` 包含 `alloc::raw_vec::RawVec<T>` 包含 `core::ptr::Unique<T>` 包含 `std::marker::PhantomData<T>` 其对`T`协变.  

| Type                          | Variance in `'a`  | Variance in `T`   |
|-------------------------------|-------------------|-------------------|
| `&'a T`                       | covariant         | covariant         |
| `&'a mut T`                   | covariant         | invariant         |
| `*const T`                    |                   | covariant         |
| `*mut T`                      |                   | invariant         |
| `[T]` and `[T; n]`            |                   | covariant         |
| `fn() -> T`                   |                   | covariant         |
| `fn(T) -> ()`                 |                   | contravariant     |
| `std::cell::UnsafeCell<T>`    |                   | invariant         |
| `std::marker::PhantomData<T>` |                   | covariant         |
| `dyn Trait<T> + 'a`           | covariant         | invariant         |


type Link1<T> = Option<NonNull<Node<T>>>; // `NonNull` is `*const T` covariant for `Node<T>`
type Link2<T> = *mut Node<T>; // invariant for `Node<T>`
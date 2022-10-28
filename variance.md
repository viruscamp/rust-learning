# 泛型型变

## 概要
`Variance`译作`型变`或`可变性`或`变体`.  

假设1 `C<T>`是一个泛型类或接口, `T`是类型参数.  
假设2 类型`Dog`是`Animal`的子类型.  

`Covariance`译作`协变`: 如果`C<Dog>`是`C<Animal>`的子类型, 那么`C<T>`对于`T`协变.  
`Contravariance`译作`逆变`: 如果`C<Animal>`是`C<Dog>`的子类型, 那么`C<T>`对于`T`逆变.  
`Invariance`译作`不变`: `C<T>`对于`T`既不是协变也不是逆变, 那么`C<T>`对于`T`不变, 或译作抗变.  

## 详解

如果以下代码合法, 那么`Dog`是`Animal`的子类型:
```java
void f1(Dog d) {
    Animal a = d;
}
```

定义示例类型继承树: 
```java
class Animal {}
class Cat extends Animal {}
class Dog extends Animal {}
class CorgiDog extends Dog {}
```

`C<T>`是一个泛型类或接口.
```java
class C<T> {}
```

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
addDog(new ArrayList<Dog>()); // OK
addDog(new ArrayList<Animal>()); // OK 逆变
```

3. Invariance 不变  
如果`C<Animal>`与`C<Dog>`不能互相替换, 那么`C<T>`对于`T`不变, 或译作抗变.
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

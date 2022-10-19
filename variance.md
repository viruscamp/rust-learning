# 泛型参数可变性

有如下类型继承树: 
```
Animal
    > Dog
        > CorgiDog
    > Cat
```
有泛型类 `C<T>`

1. Covariance 协变
  如果某个泛型参数类型可以由其派生类替换，那么这个类型就是支持协变的。
  如果`C<Dog>`是`C<Animal>`的子类型, 那么`C<T>`支持协变.
  Java写法: 实际类型`ArrayList<Dog>`可以替换类型`List<? extends Animal>`
  C#写法: `interface IEnumerable<out T>`, 实际类型`IEnumerable<Dog>`可以替换类型`IEnumerable<Animal>`
```java
// 想做的事
List<Animal> al = new ArrayList<Dog>();
Animal a = al.get(0);

// 正确做法
List<? extends Animal> al = new ArrayList<Dog>(); // 正确
Animal a = al.get(0);

// 协变类型用法
//List<Animal> b = new ArrayList<Dog>(); // 编译错误
List<? extends Animal> l = new ArrayList<Dog>(); // 正确

//Dog d = l.get(0); // 编译错误
Animal a = l.get(0);

//l.add(a); // 编译错误
//l.add(new Cat()); // 编译错误
//l.add(new Dog()); // 编译错误

// 泛型 返回值 错误 需要协变
List<Animal> getList() {
    return new ArrayList<Dog>(); // 编译错误
}

// 泛型协变 返回值
List<? extends Animal> getList() {
    return new ArrayList<Dog>();
}
Animal a = getList().get(0); // 目标

// 泛型协变 参数
void checkAnimals(List<Animal> animals) {
    for(Animal a : animals) {
        a.check();
    }
    //animals.add(new Cat()); // 可以编译通过
}
checkAnimals(new ArayyList<Dog>()); // 类型错误

// 泛型协变 参数 类型正确
void checkAnimals(List<? extends Animal> animals) {
    for(Animal a : animals) {
        a.check();
    }
    //animals.add(new Cat()); // 应该无法编译
}
checkAnimals(new ArayyList<Dog>()); // 类型正确
```
2. Contravariance 逆变
  如果某个泛型参数类型可以由其基类替换，那么这个类就是支持逆变的。
  如果`C<Animal>`是`C<Dog>`的子类型, 那么`C<T>`支持逆变.
  Java写法: 实际类型`ArrayList<Animal>`可以替换类型`List<? super Dog>`
  C#写法: `interface IComparer<in T>`, 实际类型`IComparer<Animal>`可以替换类型`IComparer<Dog>`
```java
// 想做的事
List<Dog> dl = new ArrayList<Animal>();
dl.add(new Dog());

// 正确做法
List<? super Dog> dl = new ArrayList<Animal>(); // 正确
dl.add(new Dog());

// 逆变类型用法
//List<Dog> b = new ArrayList<Animal>(); // 错误的
List<? super Dog> l = new ArrayList<Animal>(); // 正确

//Dog d = l.get(0); // 编译错误
//Animal a = l.get(0); // 编译错误

//l.add(a); // 编译错误
//l.add(new Cat()); // 编译错误
l.add(new Dog()); // 正确


// 泛型 错误的
void addDog(List<Animal> cl) {
    cl.add(new Dog()); // 需要逆变
}
addDog(new ArrayList<Cat>()); // 运行错误

// 泛型逆变
void addDog(List<? super Dog> cl) {
    cl.add(new Dog());
}
addDog(new ArrayList<Dog>()); // OK
addDog(new ArrayList<Animal>()); // OK 
//addDog(new List<Cat>()); // 编译错误
```
3. Invariance 不变: 
如果`C<Animal>`与`C<Dog>`互相不能替换, 那么`C<T>`是不变的.
```java
void useList(List<Dog> cl) {
    Dog d = cl.get(0); // List<Animal> 不能用
    cl.add(new Dog()); // List<CorgiDog> 不能用
}
useList(new ArrayList<Animal>()); // 错误
useList(new ArrayList<CorgiDog>()); // 错误
```

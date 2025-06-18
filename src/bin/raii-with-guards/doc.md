# RAII with Guards

## 설명
RAII는 “Resource Acquisition is Initialisation(자원의 획득이 초기화다)”의 약자로, 이름은 별로 좋지 않습니다. 이 패턴의 핵심은 자원의 초기화를 객체의 생성자(constructor)에서 수행하고, 정리는 소멸자(destructor)에서 수행한다는 것입니다. Rust에서는 이 패턴이 확장되어, 어떤 자원의 보호자로서 RAII 객체를 사용하고, 타입 시스템을 통해 항상 이 보호자 객체를 통해서만 자원에 접근하도록 보장합니다.

## 예제
`Mutex guard`는 표준 라이브러리에서 이 패턴(RAII)의 고전적인 예시입니다. (아래는 실제 구현을 단순화한 버전입니다:)

```rust
use std::ops::Deref;

struct Foo {}

struct Mutex<T> {
    // We keep a reference to our data: T here.
    //..
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
    //..
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        //..

        // MutexGuard keeps a reference to self
        MutexGuard {
            data: self,
            //..
        }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Unlock the underlying OS mutex.
        //..
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
              // The borrow checker ensures we can't store a reference to the underlying
              // Foo which will outlive the guard xx.

    // x is unlocked when we exit this function and xx's destructor is executed.
}
```

## 동기
자원을 사용한 후 반드시 정리해야 할 경우, RAII를 이용해 그 정리 작업을 수행할 수 있습니다. 만약 자원이 정리된 후에 접근하는 것이 오류라면, 이 패턴을 사용하여 그러한 오류를 방지할 수 있습니다.

## 장점
자원이 정리되지 않은 채로 남겨지는 오류와, 자원이 정리된 후에 사용되는 오류를 모두 방지합니다.

## 논의
RAII는 자원이 적절히 해제되거나 정리되도록 보장하는 데 유용한 패턴입니다. Rust에서는 borrow checker를 활용하여, 자원이 정리된 이후에 사용되는 것으로 인한 오류를 정적으로 방지할 수 있습니다.

borrow checker의 핵심 목적은 어떤 데이터에 대한 참조가 그 데이터보다 오래 살아남지 않도록 보장하는 것입니다. RAII 가드 패턴이 작동하는 이유는, 가드 객체가 기반 자원에 대한 참조를 포함하고 있으며, 오직 그 참조만을 외부에 노출하기 때문입니다. Rust는 가드가 기반 자원보다 오래 살아남을 수 없도록 보장하며, 가드를 통해 노출된 자원에 대한 참조도 가드보다 오래 살아남을 수 없게 합니다. 이 동작 원리를 이해하려면, lifetime 생략이 없는 `deref`의 시그니처를 살펴보는 것이 도움이 됩니다:
```rust
fn deref<'a>(&'a self) -> &'a T {
    //..
}
```
반환된 자원에 대한 참조는 `self`와 동일한 라이프타임 `'a`를 가집니다. 따라서 borrow checker는 `T`에 대한 참조의 수명이 `self`의 수명보다 짧다는 것을 보장합니다.

이때 주의할 점은, `Deref`를 구현하는 것이 이 패턴의 핵심은 아니라는 것입니다. `Deref`는 단지 가드 객체를 더 편리하게 사용할 수 있도록 해줄 뿐입니다. 가드에 `get` 메서드를 구현하는 것도 동일하게 잘 동작합니다.


## 참고자료
[Finalisation in destructors idiom](https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html)  
RAII is common pattern in C++:[cppreference.com](http://en.cppreference.com/w/cpp/language/raii)[wikipedia](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization)

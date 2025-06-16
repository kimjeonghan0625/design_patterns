# Newtype Pattern
어떤 경우에는 단순한 타입 별칭(type alias)만으로는 충분하지 않아서, 특정 타입이 다른 타입처럼 동작하거나 컴파일 시점에 특정 동작을 강제하고 싶을 수 있습니다.

예를 들어 보안상의 이유(예: 비밀번호)로 `String`에 대해 사용자 정의 `Display` 구현을 만들고 싶다고 가정해봅시다.

이런 경우에는 **타입 안전성**과 **캡슐화**를 제공하기 위해 Newtype 패턴을 사용할 수 있습니다.

## 설명
단일 필드를 갖는 튜플 구조체(tuple struct)를 사용하여 어떤 타입에 대한 불투명한(opaque) 래퍼(wrapper)를 만드세요.
이렇게 하면 기존 타입에 대한 별칭(alias)이 아니라 새로운 타입이 생성됩니다. (`type` 항목과는 다릅니다.)

## 예제
```rust
use std::fmt::Display;

// `String`의 `Display` 트레이트를 오버라이드하기 위해 Newtype인 `Password`를 생성한다.
struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

fn main() {
    let unsecured_password: String = "ThisIsMyPassword".to_string();
    let secured_password: Password = Password(unsecured_password.clone());
    println!("unsecured_password: {unsecured_password}");
    println!("secured_password: {secured_password}");
}
```
```
unsecured_password: ThisIsMyPassword
secured_password: ****************
```

## 동기
Newtype을 사용하는 주된 이유는 \*\*추상화(abstraction)\*\*입니다.
이는 타입 간에 구현 세부사항을 공유하면서도 인터페이스는 정확하게 제어할 수 있게 해줍니다.
API에서 구현 타입을 그대로 노출하는 대신 Newtype을 사용하면, 구현을 **기존 코드와 호환되게 변경**할 수 있는 유연성을 가질 수 있습니다.

Newtype은 **단위를 구분**하는 데에도 사용할 수 있습니다.
예를 들어 `f64`를 감싸서 `Miles`와 `Kilometres`처럼 서로 구분 가능한 타입으로 만들 수 있습니다.

## 장점
래핑된 타입(wrapped type)과 이를 감싼 래퍼 타입(wrapper type)은 **서로 타입 호환되지 않습니다** (이는 `type` 별칭과는 다릅니다).
따라서 Newtype을 사용하면, **사용자가 두 타입을 혼동할 일이 없습니다.**

Newtype은 \*\*제로-코스트 추상화(zero-cost abstraction)\*\*입니다 — 즉, **런타임 오버헤드가 전혀 없습니다.**

또한, 필드가 기본적으로 private이기 때문에 **프라이버시 시스템이 래핑된 타입에 대한 직접 접근을 차단**해줍니다.

## 단점
Newtype의 단점은 (특히 타입 별칭과 비교했을 때) **언어 차원의 특별한 지원이 없다는 점**입니다.
즉, **보일러플레이트 코드가 많아질 수 있습니다.**

래핑된 타입의 메서드를 노출하려면 **노출하려는 모든 메서드에 대해 '중계(pass through)' 메서드**를 작성해야 하며,
래퍼 타입에서도 동작하게 하고 싶은 트레이트마다 **직접 `impl` 블록을 작성**해야 합니다.

## 논의
Rust 코드에서 Newtype은 매우 흔하게 사용됩니다. 가장 일반적인 용도는 **추상화** 또는 **단위를 표현**하기 위한 것이지만, 다음과 같은 다른 이유로도 사용됩니다:

* **기능 제한**: 노출되는 함수나 구현되는 트레이트를 줄이기 위해 (예: 어떤 기능을 의도적으로 감추기 위해).
* **복사(Copy) 가능한 타입을 이동(Move) 타입으로 만들기 위해**: 예를 들어, `u32`는 `Copy`이지만 이를 감싸는 Newtype은 `Copy`를 구현하지 않으면 `Move`로 동작하게 할 수 있습니다.
* **내부 타입을 숨기기 위한 추상화**: 더 구체적인 타입을 제공함으로써 내부 구조를 감출 수 있습니다. 예:

  ```rust
  pub struct Foo(Bar<T1, T2>);
  ```

  여기서 `Bar`는 공개된 제네릭 타입일 수 있고, `T1`, `T2`는 내부 구현에만 쓰이는 타입입니다. 우리의 모듈을 사용하는 외부 사용자에게는 `Foo`가 `Bar`를 기반으로 구현되었는지 알릴 필요가 없고, 실제로 숨기고자 하는 것은 `T1`과 `T2`, 그리고 이들이 `Bar`에서 어떻게 사용되는지입니다.

이러한 Newtype 패턴은 **의도한 인터페이스만 노출하고, 내부 구현의 유연성을 유지**하는 데 매우 유용합니다.

## 참고 자료
- [Advanced Types in the book](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=newtype#using-the-newtype-pattern-for-type-safety-and-abstraction)
- [Advanced Types in the book - 한국어](https://doc.rust-kr.org/ch19-04-advanced-types.html)
- [Newtypes in Haskell](https://wiki.haskell.org/Newtype)
- [Type aliases](https://doc.rust-lang.org/stable/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases)
- [derive_more](https://crates.io/crates/derive_more), a crate for deriving many builtin traits on newtypes.
- [The Newtype Pattern In Rust](https://web.archive.org/web/20230519162111/https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)

# Strategy (aka Policy)

## 설명
[전략(Strategy) 디자인 패턴](https://en.wikipedia.org/wiki/Strategy_pattern)은 관심사의 분리를 가능하게 하는 기법입니다. 또한 [의존성 역전(Dependency Inversion)](https://en.wikipedia.org/wiki/Dependency_inversion_principle)을 통해 소프트웨어 모듈을 분리할 수 있게 해줍니다.

Strategy 패턴의 기본 아이디어는, 특정 문제를 해결하는 알고리즘이 있을 때 알고리즘의 뼈대만 추상적인 수준에서 정의하고, 구체적인 알고리즘 구현은 별도의 부분으로 분리하는 것입니다.

이렇게 하면 알고리즘을 사용하는 클라이언트는 특정 구현을 선택할 수 있고, 전체 알고리즘의 흐름은 동일하게 유지됩니다. 다시 말해, 클래스의 추상 명세는 파생 클래스의 구체적인 구현에 의존하지 않지만, 구체적인 구현은 추상 명세를 따라야 합니다. 이것이 바로 "의존성 역전"이라고 부르는 이유입니다.


## 동기
우리가 매달 보고서를 생성하는 프로젝트를 진행하고 있다고 상상해보세요. 우리는 보고서를 다양한 형식(전략)으로 생성해야 합니다. 예를 들어 `JSON`이나 `Plain Text` 형식이 될 수 있습니다. 하지만 시간이 지나면서 상황이 달라질 수 있고, 앞으로 어떤 요구사항이 생길지 알 수 없습니다. 예를 들어, 완전히 새로운 형식으로 보고서를 생성해야 하거나, 기존 형식 중 하나를 수정해야 할 수도 있습니다.

## 예시
이 예제에서 우리의 불변(또는 추상화) 요소는 `Formatter`와 `Report`이고, `Text`와 `Json`은 전략 구조체입니다. 이러한 전략들은 `Formatter` 트레이트를 구현해야 합니다.

```rust
use std::collections::HashMap;

type Data = HashMap<String, u32>;

trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

struct Report;

impl Report {
    // Write should be used but we kept it as String to ignore error handling
    fn generate<T: Formatter>(g: T, s: &mut String) {
        // backend operations...
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate report
        g.format(&data, s);
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{k} {v}\n");
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop(); // remove extra , at the end
        }
        buf.push(']');
    }
}

fn main() {
    let mut s = String::from("");
    Report::generate(Text, &mut s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear(); // reuse the same buffer
    Report::generate(Json, &mut s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
```

## 장점

주요 장점은 관심사의 분리입니다. 예를 들어, 이 경우 `Report`는 `Json`과 `Text`의 구체적인 구현에 대해 아무것도 알 필요가 없으며, 출력 구현체 역시 데이터가 어떻게 전처리되고 저장되고 가져오는지 신경 쓸 필요가 없습니다. 이들이 알아야 할 것은 어떤 트레이트를 구현해야 하는지와 결과를 처리하는 구체적인 알고리즘을 정의하는 해당 트레이트의 메서드(`Formatter`와 `format(...)`)뿐입니다.

## 단점

각 전략마다 최소 하나의 모듈을 구현해야 하므로, 전략의 수가 많아질수록 모듈의 수도 증가합니다. 선택할 수 있는 전략이 많을 경우, 사용자들은 각 전략이 어떻게 다른지 이해해야 하는 부담이 생깁니다.

## 논의

이전 예제에서는 모든 전략이 하나의 파일에 구현되어 있습니다. 다양한 전략을 제공하는 방법에는 다음과 같은 것들이 있습니다:

- 모든 전략을 하나의 파일에 구현 (이 예제처럼, 모듈로 분리하는 것과 유사)
- 모듈로 분리, 예: `formatter::json` 모듈, `formatter::text` 모듈
- 컴파일러 feature 플래그 사용, 예: `json` feature, `text` feature
- 크레이트로 분리, 예: `json` 크레이트, `text` 크레이트

Serde 크레이트는 `Strategy` 패턴이 실제로 적용된 좋은 예시입니다. Serde는
[직렬화 동작의 완전한 커스터마이즈](https://serde.rs/custom-serialization.html)를 위해
직접 `Serialize`와 `Deserialize` 트레이트를 구현할 수 있게 해줍니다. 예를 들어,
`serde_json`을 `serde_cbor`로 쉽게 교체할 수 있는데, 이들은 유사한 메서드를 제공하기 때문입니다.
이러한 구조 덕분에 보조 크레이트인 `serde_transcode`가 훨씬 더 유용하고 사용하기 쉬워집니다.

하지만 Rust에서 이 패턴을 설계할 때 반드시 트레이트를 사용할 필요는 없습니다.

다음의 간단한 예제는 Rust의 `클로저`를 사용하여 Strategy 패턴의 아이디어를 보여줍니다:

```rust
struct Adder;
impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

fn main() {
    let arith_adder = |x, y| x + y;
    let bool_adder = |x, y| {
        if x == 1 || y == 1 {
            1
        } else {
            0
        }
    };
    let custom_adder = |x, y| 2 * x + y;

    assert_eq!(9, Adder::add(4, 5, arith_adder));
    assert_eq!(0, Adder::add(0, 0, bool_adder));
    assert_eq!(5, Adder::add(1, 3, custom_adder));
}
```

사실 Rust는 이미 이 아이디어를 `Option`의 `map` 메서드에서 사용하고 있습니다:

```rust
fn main() {
    let val = Some("Rust");

    let len_strategy = |s: &str| s.len();
    assert_eq!(4, val.map(len_strategy).unwrap());

    let first_byte_strategy = |s: &str| s.bytes().next().unwrap();
    assert_eq!(82, val.map(first_byte_strategy).unwrap());
}
```
## 참고 자료

- [Strategy Pattern](https://en.wikipedia.org/wiki/Strategy_pattern)
- [Dependency Injection](https://en.wikipedia.org/wiki/Dependency_injection)
- [Policy Based Design](https://en.wikipedia.org/wiki/Modern_C++_Design#Policy-based_design)
- [Implementing a TCP server for Space Applications in Rust using the Strategy Pattern](https://web.archive.org/web/20231003171500/https://robamu.github.io/posts/rust-strategy-pattern/)
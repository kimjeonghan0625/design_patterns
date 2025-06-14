# Interpreter 패턴

## 설명

만약 어떤 문제가 매우 자주 발생하고 그 문제를 해결하기 위해 긴 반복적인 절차가 필요하다면,
그 문제의 인스턴스들을 **간단한 언어로 표현**할 수 있고,
**해석기(interpreter) 객체가** 그 언어로 작성된 문장을 해석하여 문제를 해결하도록 만들 수 있습니다.

기본적으로, 이 패턴에서는 다음과 같은 구성을 정의합니다:

* **도메인 특화 언어 (DSL, Domain Specific Language)**
* **그 언어의 문법 (Grammar)**
* **문제 인스턴스를 해결할 수 있는 해석기 (Interpreter)**

## 동기
**동기 (Motivation)**

우리의 목표는 간단한 수학 표현식을 \*\*후위 표기법(Postfix, 또는 Reverse Polish Notation)\*\*으로 변환하는 것입니다.
단순화를 위해, 표현식은 **숫자 0부터 9까지**와 \*\*두 개의 연산자 +, -\*\*만 사용한다고 가정합니다.

예를 들어,
표현식 `2 + 4`는 후위 표기법으로 다음과 같이 변환됩니다:

```
2 4 +
```

## 후위 표기법으로의 변환을 위한 문맥 자유 문법 (Context Free Grammar)

우리는 \*\*중위 표현식(infix expression)\*\*을 \*\*후위 표현식(postfix expression)\*\*으로 변환하려고 합니다.
이를 위해 다음과 같은 문맥 자유 문법(CFG, Context-Free Grammar)을 정의할 수 있습니다:


### 📌 구성 요소:

* **터미널 기호 (Terminal symbols):**

  ```
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, +, -
  ```

* **비터미널 기호 (Non-terminal symbols):**

  ```
  exp, term
  ```

* **시작 기호 (Start symbol):**

  ```
  exp
  ```


### 📌 생성 규칙 (Production rules):

```
exp  -> exp + term
exp  -> exp - term
exp  -> term
term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```
```antlr
// ANTLR Style 문법 정의

// Parser rules
expr
    : expr PLUS term 
    | expr MINUS term
    | term           
    ;

term
    : DIGIT
    ;

// Lexer(Tokenizer) rules
PLUS  : '+' ;
MINUS : '-' ;
DIGIT : [0-9] ;
```

### 🔁 주의 (NOTE):

이 문법은 현재 \*\*좌측 재귀(left recursion)\*\*를 포함하고 있습니다.
좌측 재귀는 파서(예: 재귀 하강 파서)에서 직접 사용할 수 없기 때문에,
이를 제거하고 **우측 재귀(right recursion)** 형태로 변형해야 할 수도 있습니다.

자세한 이론 및 기법은 다음 참고 문헌에서 확인할 수 있습니다:

> **Compilers: Principles, Techniques, and Tools**
> (일명 "드래곤 북", Dragon Book)

## 해결책

우리는 단순히 재귀 하강 파서(recursive descent parser)를 구현합니다. 단순함을 위해, 표현식이 문법적으로 잘못된 경우(예: `2-34` 또는 `2+5-` 는 문법 정의에 따라 잘못된 표현식입니다) 코드가 패닉을 일으킵니다.

```rust
pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("예상치 못한 기호 '{op}'");
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("예상치 못한 기호 '{ch}'"),
            None => panic!("예상치 못한 문자열의 끝"),
        }
    }
}

pub fn main() {
    let mut intr = Interpreter::new("2+3");
    let mut postfix = String::new();
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "23+");

    intr = Interpreter::new("1-2+3-4");
    postfix.clear();
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "12-3+4-");
}
```

## 논의

Interpreter 디자인 패턴은 흔히 형식 언어(formal language)를 위한 문법을 설계하고, 이 문법을 파싱하는 파서를 구현하는 것이라고 오해될 수 있습니다. 하지만 실제로 이 패턴은 **문제 인스턴스를 더 구체적인 방법으로 표현**하고, **그 인스턴스를 해결하는 함수/클래스/구조체를 구현하는 것**에 관한 것입니다.

Rust 언어에는 `macro_rules!`라는 매크로 시스템이 있어서 **특정한 문법을 정의하고**, **이 문법이 소스 코드로 어떻게 확장될지를 정할 수 있습니다**.

다음은 n차원 벡터의 유클리드 길이를 계산하는 간단한 `macro_rules!` 예제입니다.
`norm!(x, 1, 2)`처럼 쓰는 것이, 값을 `Vec`으로 묶어 함수에 넘기는 것보다 더 **표현하기 쉽고 효율적일 수 있습니다.**

```rust
macro_rules! norm {
    ($($element:expr),*) => {
        {
            let mut n = 0.0;
            $(
                n += ($element as f64) * ($element as f64);
            )*
            n.sqrt()
        }
    };
}

fn main() {
    let x = -3f64;
    let y = 4f64;

    assert_eq!(3f64, norm!(x));
    assert_eq!(5f64, norm!(x, y));
    assert_eq!(0f64, norm!(0, 0, 0));
    assert_eq!(1f64, norm!(0.5, -0.5, 0.5, -0.5));
}
```

이처럼 Rust에서는 Interpreter 패턴의 아이디어를 **매크로로 추상화**해 활용할 수도 있습니다.

### 참고 문헌
- [Interpreter Pattern](https://en.wikipedia.org/wiki/Interpreter_pattern)
- [Context-Free Grammar](https://en.wikipedia.org/wiki/Context-free_grammar)
- [macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)
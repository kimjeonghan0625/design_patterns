use std::ops::Deref;

struct Foo;

impl Foo {
    fn foo(&self) {
        println!("Foo::foo() called");
    }
}

struct Mutex<T> {
    data: T,
}

struct MutexGuard<'a, T> {
    data: &'a T,
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    fn new(data: T) -> Self {
        Mutex { data }
    }

    fn lock(&self) -> MutexGuard<'_, T> {
        println!("Lock acquired");
        MutexGuard { data: &self.data }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        println!("Lock released");
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

fn baz(x: &Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
    // x is unlocked when `xx` goes out of scope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex() {
        let m = Mutex::new(Foo);
        baz(&m);
    }
}

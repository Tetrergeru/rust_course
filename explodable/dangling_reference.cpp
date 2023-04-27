#include <iostream>

struct Foo {
    int& x;
};

Foo make_foo(int x) {
    return Foo{x};
}

void test_1() {
    int x = 42;
    Foo foo {x};

    // Что выведется?
    std::cout << foo.x << std::endl;   
}

void test_2() {
    Foo foo = make_foo(42);
    
    // Что выведется?
    std::cout << foo.x << std::endl;   
}

void test_3() {
    Foo foo_1 = make_foo(42);
    Foo foo_2 = make_foo(43);
    
    // Что выведется?
    std::cout << foo_1.x << std::endl;   
    std::cout << foo_2.x << std::endl;   
}

int main() {
    // test_1();
    // test_2();
    test_3();
}
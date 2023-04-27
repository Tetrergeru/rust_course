#include <iostream>
#include <memory>

struct Data {
    int * ptr, data;
    
    Data(int v) { ptr = new int(v); data = v;}
    ~Data() { delete ptr; }

    void set(int v) { *ptr = v; data = v; }

    int get() {
        if (*ptr != data)
            throw std::exception();
        return data;
    }
};

void do_sth(Data data) {
    data.set(666);
}

int main()
{
    Data data = {42};
    std::cout << data.get() << std::endl;
    do_sth(data);
    std::cout << data.get() << std::endl;
}
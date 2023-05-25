#include <vector>
#include <iostream>
#include <thread>

using namespace std;

int main()
{
    double data = 0;

    auto vec = vector<thread>();
    for (auto i = 0; i < 1000; i++) 
    {
        vec.push_back(thread([&data](double i) {
            for (auto j = 0; j < 500; j++) {
                data++;
            }
        }, i));
    }

    for (auto &t : vec)
        t.join();

    cout << data << endl;
}
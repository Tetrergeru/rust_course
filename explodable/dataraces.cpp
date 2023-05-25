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
            data = i;
        }, i));
    }

    for (auto &t : vec)
        t.join();

    cout << data << endl;
}
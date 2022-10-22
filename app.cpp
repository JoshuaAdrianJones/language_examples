// The C++ standard version:
#include <cstdio>
#include <iostream>
class HelloClass {        // The class
  public:              // Access specifier
    void hello() {  // Method/function defined inside the class
      std::cout << "Hello World from inside an c++ object!\n";
    }
};

int main()
{
    printf("Hello, world from C++!\n");
    HelloClass greeter;
    greeter.hello();
    return 0;
}

#include "hello.h"
#include <iostream>

int main(){
  int a = 5;
  double b = 2.5;
  int result = myFunction(a, b);
  std::cout << "Result: " << result << std::endl;
  return 0;
}

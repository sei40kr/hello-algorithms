cmake_minimum_required(VERSION 2.8)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_C_STANDARD 99)

add_executable(fibonacci-numbers fibonacci-numbers/fibonacci-numbers.cpp)
add_executable(gcd-lcm gcd-lcm/gcd-lcm.cpp)
add_executable(grundy-number grundy-number/grundy-number.cpp)
add_executable(lis lis/lis.cpp)
add_executable(lis-fast lis-fast/lis-fast.cpp)

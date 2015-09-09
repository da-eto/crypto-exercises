cmake_minimum_required(VERSION 3.3)
project(simple_ascii_xor)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES main.cpp)
add_executable(simple_ascii_xor ${SOURCE_FILES})
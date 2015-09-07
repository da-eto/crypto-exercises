cmake_minimum_required(VERSION 3.3)
project(file_stream_signature)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES main.cpp)
add_executable(file_stream_signature ${SOURCE_FILES})
target_link_libraries(file_stream_signature -lssl -lcrypto)

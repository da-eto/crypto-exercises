cmake_minimum_required(VERSION 2.8)
project(dlog)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake)
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99")

find_package(GMP REQUIRED)

add_executable(dlog-c main.c)
target_link_libraries(dlog-c ${GMP_LIBRARIES})

#add_executable(dlog main.cpp)

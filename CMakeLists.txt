cmake_minimum_required(VERSION 3.29.0)
project(najm VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

include_directories(include)
file(GLOB SOURCES "src/*.cpp")
add_executable(najm ${SOURCES})


set_property(TARGET najm PROPERTY CXX_STANDARD 20)

cmake_minimum_required(VERSION 3.0)

project(cpp-rust-cpp)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_language(Rust)
include(CMakeCargo)

add_subdirectory(cpp)
add_subdirectory(src)

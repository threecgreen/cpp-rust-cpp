add_executable(cpp-rust-cpp bindings.h logger.cpp logger.h main.cpp)

target_link_libraries(cpp-rust-cpp impl-lib pthread dl)

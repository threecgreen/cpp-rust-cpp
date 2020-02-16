add_executable(salty-rust main.cpp)

target_link_libraries(salty-rust impl-lib pthread dl)

#include <cstdint>
#include <iostream>

#include "bindings.h"
#include "logger.h"

void EventLoop()
{
  Logger logger();
  ffi::Demo demoAlgo = ffi::create(&logger);
  ffi::on_register(&demoAlgo);
  ffi::on_system_start(&demoAlgo);
  for (size_t i = 0; i < 10; ++i)
  {
    ffi::main_thread_event(&demoAlgo);
  }
  ffi::background_thread_event(&demoAlgo);
  ffi::on_system_stop(&demoAlgo);
  ffi::on_unregister(&demoAlgo);
}

int main(int argc, char** argv)
{
  std::cout << "Starting loop" << std::endl;
  EventLoop();
	return 0;
}

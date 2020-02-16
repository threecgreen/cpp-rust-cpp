#include "logger.h"

#include <iostream>
#include <string>
#include <unordered_map>

static std::unordered_map<Level, std::string> kLevelToName =
{
  {Level::Debug, "Debug"},
  {Level::Info, "Info"},
  {Level::Warning, "Warning"},
  {Level::Error, "Error"}
};

void Logger::Persist(Level level, char const* msg)
{
  std::cout << kLevelToName[level] << ": " << msg << std::endl;
}

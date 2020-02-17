#pragma once

#include <cstdint>

// namespace Cpp
// {
enum class Level : uint8_t
{
  Debug,
  Info,
  Warning,
  Error
};

class Logger
{
 public:
  Logger() = default;

  void Persist(Level level, char const* msg);
};
// }   // namespace Cpp

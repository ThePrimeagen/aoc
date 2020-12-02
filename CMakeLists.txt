cmake_minimum_required(VERSION 3.10)

# set the project name
project(honorable_aoc VERSION 1.0)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Check for correct compiler
# Using C compiler GNUCXX for c++ compiler
IF(CMAKE_COMPILER_IS_GNUCC)
  MESSAGE(STATUS "=== GCC C COMPILER DETECTED")
  SET(CMAKE_C_FLAGS "-ggdb -D_DEBUG -Wextra -Wall -Wunreachable-code -O0 -D_LARGEFILE64_SOURCE")
ENDIF(CMAKE_COMPILER_IS_GNUCC)

add_subdirectory(src)

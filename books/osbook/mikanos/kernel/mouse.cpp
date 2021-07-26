#include "mouse.hpp"

#include "graphics.hpp"

namespace {

const int kMouseCusorWidth = 15;
const int kMouseCusorHeight = 24;
const char mouse_cursor_shape[kMouseCusorHeight][kMouseCusorWidth + 1] = {
  "@              ",
  "@@             ",
  "@.@            ",
  "@..@           ",
  "@...@          ",
  "@....@         ",
  "@.....@        ",
  "@......@       ",
  "@.......@      ",
  "@........@     ",
  "@.........@    ",
  "@..........@   ",
  "@...........@  ",
  "@............@ ",
  "@......@@@@@@@@",
  "@......@       ",
  "@....@@.@      ",
  "@...@ @.@      ",
  "@..@   @.@     ",
  "@.@    @.@     ",
  "@@      @.@    ",
  "@       @.@    ",
  "         @.@   ",
  "         @@@   ",
};

void DrawMouseCursor(PixelWriter *pixel_writer, Vector2D<int> positon) {
  for (int dy = 0; dy < kMouseCusorHeight; ++dy) {
    for (int dx = 0; dx < kMouseCusorWidth; ++dx) {
      if (mouse_cursor_shape[dy][dx] == '@') {
        pixel_writer->Write(positon.x + dx, positon.y + dy, {0, 0, 0});
      } else {
        pixel_writer->Write(positon.x + dx, positon.y + dy, {255, 255, 255});
      }
    }
  }
}

void EraseMouseCursor(PixelWriter *pixel_writer, Vector2D<int> positon,
                      PixelColor erase_color) {
  for (int dy = 0; dy < kMouseCusorHeight; ++dy) {
    for (int dx = 0; dx < kMouseCusorWidth; ++dx) {
      if (mouse_cursor_shape[dy][dx] != ' ') {
        pixel_writer->Write(positon.x + dx, positon.y + dy, erase_color);
      }
    }
  }
}


MouseCursor::MouseCursor(PixelWriter* writer, PixelColor erase_color,
                         Vector2D<int> initial_position)
    : pixel_writer_{writer},
      erase_color_{erase_color},
      position_{initial_position} {
  DrawMouseCursor(pixel_writer_, position_);
}

void MouseCursor::MoveRelative(Vector2D<int> displacement) {
  EraseMouseCursor(pixel_writer_, position_, erase_color_);
  position_ += displacement;
  DrawMouseCursor(pixel_writer_, position_);
}

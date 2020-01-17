#include <SDL2/SDL.h>
#include <stdio.h>

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;

int main(int argc, char* args[]) {

  SDL_Window* window = NULL;
  SDL_Renderer *renderer = NULL;

  //Initialize SDL
  if(SDL_Init( SDL_INIT_VIDEO) < 0 ) {
    printf( "SDL could not initialize! SDL_Error: %s\n", SDL_GetError() );
  } else {
    //Create window
    window = SDL_CreateWindow("Hello SDL", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, 0);
    if(window == NULL) {
      printf( "Window could not be created! SDL_Error: %s\n", SDL_GetError() );
    } else {

      renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);

      SDL_RenderClear(renderer);
      SDL_RenderPresent(renderer);

      SDL_Delay(3000);
    }
  }

  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);

  SDL_Quit();

  return 0;
}

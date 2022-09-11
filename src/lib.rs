use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc= wee_alloc::WeeAlloc::INIT;

struct SnakeCell(usize);

#[derive(std::cmp::PartialEq)]
enum Direction{
    Up,
    Right,
    Down, 
    Left
}

struct Snake{
    body: Vec<SnakeCell>,
     direction: Direction
}


impl Snake{
    fn new(spawn_index: usize)->Snake{
        Snake{
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Up
        }


    }
}

#[wasm_bindgen]
pub struct World{
     width: usize,
     snake: Snake,
     size: usize
}
#[wasm_bindgen]
impl World{
    pub fn new(width: usize, snake_idx: usize)->World{
        World{
            width,
            size: width*width,
            snake: Snake::new(snake_idx)
        }

    }
    pub fn width(&self)->usize{
        self.width
    }
    pub fn snake_head_idx(&self)->usize{
        self.snake.body[0].0
    }
    pub fn update(&mut self){
        let snake_idx=self.snake_head_idx();
        let row=snake_idx/self.width();
        let col=snake_idx%self.width();

        if self.snake.direction==Direction::Right{
            let next_col=(col+1)%self.width();
            self.snake.body[0].0=(row*self.width())+next_col;

        }
        if self.snake.direction==Direction::Left{
            let next_col=(col-1)&self.width();
            self.snake.body[0].0=(row*self.width())+next_col;
        }
        if self.snake.direction==Direction::Up{
            let next_row=(row-1)%self.width();
            self.snake.body[0].0=(next_row*self.width())+col;
        }
        if self.snake.direction==Direction::Down{
            let next_row=(row+1)%self.width();
            self.snake.body[0].0=(next_row*self.width())+col;
        }
    }
}


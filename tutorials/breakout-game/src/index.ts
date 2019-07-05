const canvas = <HTMLCanvasElement>document.getElementById('myCanvas');
const ctx: CanvasRenderingContext2D = canvas.getContext('2d');

let x: number = canvas.width / 2;
let y: number = canvas.height - 30;
let dx: number = 2;
let dy: number = -2;
const ballRadius = 10;

const paddleHeight: number = 10;
const paddleWidth: number = 75;
let paddleX: number = (canvas.width - paddleWidth) / 2;

let rightPressed = false;
let leftPressed = false

let brickRowCount = 3;
let brickColumnCount = 5;
let brickWidth = 75;
let brickHeight = 20;
let brickPadding = 10;
let brickOffsetTop = 30;
let brickOffsetLeft = 30;

interface Brick {
  x: number;
  y: number;
}

let bricks: Brick[][] = [];
for (let c = 0; c < brickColumnCount; c++) {
  bricks[c] = [];
  for (let r = 0; r < brickRowCount; r++) {
    bricks[c][r] = { x: 0, y: 0 };
  }
}


const keyDownHandler = (e: KeyboardEvent): void => {
  if (e.key === "Right" || e.key === "ArrowRight") {
    console.log("right down");
    rightPressed = true;
  }
  if (e.key === "Left" || e.key === "ArrowLeft") {
    console.log("left down");
    leftPressed = true;
  }
};

const keyUpHandler = (e: KeyboardEvent): void => {
  if (e.key === "Right" || e.key === "ArrowRight") {
    console.log("right up");
    rightPressed = false;
  }
  if (e.key === "Left" || e.key === "ArrowLeft") {
    console.log("left up");
    leftPressed = false;
  }
};

const drawPaddle = (): void => {
  ctx.beginPath();
  ctx.rect(paddleX, canvas.height - paddleHeight, paddleWidth, paddleHeight);
  ctx.fillStyle = "#0095DD";
  ctx.fill();
  ctx.closePath();
}

const drawBall = (): void => {
  ctx.beginPath();
  ctx.arc(x, y, ballRadius, 0, Math.PI * 2);
  ctx.fillStyle = "#0095DD";
  ctx.fill();
  ctx.closePath();
}


const draw = (): void => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawPaddle();
  drawBall()

  if (x + dx > canvas.width - ballRadius || x + dx < ballRadius) {
    dx = -dx;
  }


  if (y + dy < ballRadius) {
    dy = -dy;
  } else if (y + dy > canvas.height - ballRadius) {
    if (x > paddleX && x < paddleX + paddleWidth) {
      dy = -dy;
    } else {
      alert('GAME OVER');
      document.location.reload();
      clearInterval(intervl);
    }
  }

  if (rightPressed && paddleX < canvas.width - paddleWidth) {
    paddleX += 7;
  }

  if (leftPressed && paddleX > 0) {
    paddleX += -7;
  }

  x += dx;
  y += dy;
};

document.addEventListener("keydown", keyDownHandler, false);
document.addEventListener("keyup", keyUpHandler, false);

const intervl = setInterval(draw, 10);

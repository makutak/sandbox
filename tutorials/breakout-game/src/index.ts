const canvas = <HTMLCanvasElement>document.getElementById('myCanvas');
const ctx: CanvasRenderingContext2D = canvas.getContext('2d');

let x: number = canvas.width / 2;
let y: number = canvas.height - 30;
let dx: number = 2;
let dy: number = -2;

function drawBall() {
  ctx.beginPath();
  ctx.arc(x, y, 10, 0, Math.PI * 2);
  ctx.fillStyle = "#0095DD";
  ctx.fill();
  ctx.closePath();
}


function draw(): void {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawBall()
  x += dx;
  y += dy;
};

setInterval(draw, 10);

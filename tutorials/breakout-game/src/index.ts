const canvas = <HTMLCanvasElement>document.getElementById('myCanvas');
const ctx: CanvasRenderingContext2D = canvas.getContext('2d');

const draw = (): void => {
  console.log("draw");
  ctx.beginPath();
  ctx.arc(50, 50, 10, 0, Math.PI * 2);
  ctx.fillStyle = "#0095DD";
  ctx.fill();
  ctx.closePath();
};

setInterval(draw, 10);

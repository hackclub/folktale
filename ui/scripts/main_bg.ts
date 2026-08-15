const canvas = document.getElementById("bg_canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

const FPS = 8;
const SCALE = 8;

let width = 0;
let height = 0;

function resize(): void {
    width = Math.ceil(window.innerWidth / SCALE);
    height = Math.ceil(window.innerHeight / SCALE);
    canvas.width = width;
    canvas.height = height;
    canvas.style.width = width * SCALE + "px";
    canvas.style.height = height * SCALE + "px";
    ctx.imageSmoothingEnabled = false;
}

let instant = 0;

function tick(): void {
    console.log("hello, world!")
    ctx.fillStyle = "#2f333f";
    ctx.fillRect(0, 0, width, height);
    instant++;
}

window.addEventListener("resize", resize);

resize();
setInterval(tick, 1000 / FPS);
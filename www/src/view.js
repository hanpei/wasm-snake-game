import { config as cfg } from './main';

export class View {
    constructor(game) {
        this.init(game);
    }

    init(game) {
        this.gameWidth = game.width();
        this.gameHeight = game.height();
        this.snake = game.get_snake();
        this.food = game.get_food();
        this.score = game.get_score();

        this.container = document.querySelector(cfg.container);
        let { width: containerWidth, height: containerHeight } =
            this.container.getBoundingClientRect();

        this.cellSize = Math.min(
            containerWidth / this.gameWidth,
            containerHeight / this.gameHeight
        );

        let canvas = document.createElement('canvas');
        canvas.setAttribute('width', this.gameWidth * this.cellSize);
        canvas.setAttribute('height', this.gameHeight * this.cellSize);
        this.container.appendChild(canvas);
        this.ctx = canvas.getContext('2d');

        this.scoreElement = document.querySelector('.score');
        this.scoreElement.innerText = this.score;
    }

    update(snake, food, score) {
        this.snake = snake;
        this.food = food;
        this.score = score;
        this.render();
    }

    render() {
        this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);

        this.drawBoard();
        this.drawFood();
        this.drawSnake();
        this.drawScore();
    }

    drawScore() {
        this.scoreElement.innerText = this.score;
    }

    drawBoard() {
        this.ctx.fillStyle = 'black';
        this.ctx.globalAlpha = 0.2;
        for (let y = 0; y <= this.gameHeight; y++) {
            for (let x = 0; x <= this.gameWidth; x++) {
                if ((x + y) % 2 === 1) {
                    this.ctx.fillRect(
                        x * this.cellSize,
                        y * this.cellSize,
                        this.cellSize,
                        this.cellSize
                    );
                }
            }
        }
        this.ctx.globalAlpha = 1;
        // for (let y = 0; y <= this.gameHeight; y++) {
        //     for (let x = 0; x <= this.gameWidth; x++) {
        //         this.ctx.font = '12px serif';
        //         this.ctx.fillStyle = '#ffffff';
        //         this.ctx.fillText(
        //             `${x}, ${y}`,
        //             x * this.cellSize,
        //             y * this.cellSize + 10
        //         );
        //     }
        // }
    }

    drawSnake() {
        this.ctx.beginPath();

        this.ctx.lineWidth = this.cellSize;
        this.ctx.lineCap = 'square';
        this.ctx.strokeStyle = cfg.snakeColor;

        this.snake.forEach((p) => {
            let x = (p.x + 0.5) * this.cellSize;
            let y = (p.y + 0.5) * this.cellSize;
            this.ctx.lineTo(x, y);
        });
        this.ctx.stroke();
    }

    drawFood() {
        const { x, y } = this.food;

        this.ctx.beginPath();
        this.ctx.fillStyle = cfg.foodColor;
        this.ctx.arc(
            x * this.cellSize + this.cellSize / 2,
            y * this.cellSize + this.cellSize / 2,
            this.cellSize / 2,
            0,
            2 * Math.PI
        );
        this.ctx.fill();
    }

    reload(game) {
        let canvas = this.container.querySelector('canvas');
        this.container.removeChild(canvas);
        this.init(game);
        this.render();
    }
}

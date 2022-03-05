import { Game, Direction } from 'wasm-snake-game';
import { View } from './view.js';
import { Controller } from './controller.js';

export const config = {
    container: '.container',
    gameWidth: 27,
    gameHeight: 25,
    foodColor: '#FF628C',
    snakeColor: '#3498db',
    speed: 1,
};

export class GameManager {
    constructor() {
        this.initGame();
        this.view = new View(this.game);
        this.bindEvents();
        this.controller = new Controller();
        this.bindResart = this.bindResart.bind(this);
    }

    initGame() {
        this.gameStatus = 'init';
        this.game = new Game(config.gameWidth, config.gameHeight, config.speed);
    }

    bindEvents() {
        window.addEventListener('resize', () => {
            this.view.reload(this.game);
        });
    }

    step() {
        if (!this.controller.gameStoped) {
            let movement = this.controller.movement;
            // console.log(movement);
            this.game.step(movement);
            this.view.update(
                this.game.get_snake(),
                this.game.get_food(),
                this.game.get_score()
            );
        }
    }

    loop() {
        if (this.game.is_over()) {
            this.view.gameOver();
            this.controller.movement = undefined;
            this.addListenToRestart();
            return;
        }
        this.timer = setTimeout(() => {
            this.step();
            requestAnimationFrame(this.loop.bind(this));
        }, 1000 / 10 / this.game.get_speed());
    }

    addListenToRestart() {
        document.addEventListener('keydown', this.bindResart);
    }

    bindResart({ code }) {
        if (code === 'Enter') {
            this.restart();
        }
    }

    removeListenToRestart() {
        document.removeEventListener('keydown', this.bindResart);
    }

    restart() {
        clearTimeout(this.timer);
        this.removeListenToRestart();

        this.game.restart();
        this.view.restart(this.game);
        this.run();
    }

    run() {
        console.log('game running... ', this.game);
        console.log('snake:  ', this.game.get_snake());
        console.log('speed:  ', this.game.get_speed());
        this.loop();
    }
}

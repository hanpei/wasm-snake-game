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
        this.game = new Game(config.gameWidth, config.gameHeight, config.speed);
        this.view = new View(this.game);
        this.bindEvents();
        this.controller = new Controller();
        window.game = this.game;
    }

    bindEvents() {
        window.addEventListener('resize', () => {
            this.view.reload(this.game);
        });
    }

    render() {
        this.view.render();
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
        this.step();
        setTimeout(() => {
            requestAnimationFrame(this.loop.bind(this));
        }, 1000 / 10 / this.game.get_speed());
    }

    run() {
        this.render();
        console.log('game running... ', this.game);
        // console.log('snake:  ', this.game.get_snake());
        this.loop();
    }
}

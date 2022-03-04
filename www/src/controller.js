import { Direction } from 'wasm-snake-game';

export class Controller {
    constructor() {
        this.gameStoped = false;
        this.addEvents();
    }

    addEvents() {
        document.addEventListener('keydown', ({ code }) => {
            switch (code) {
                case 'KeyW':
                    this.movement = Direction.Up;
                    break;
                case 'KeyS':
                    this.movement = Direction.Down;

                    break;
                case 'KeyA':
                    this.movement = Direction.Left;

                    break;
                case 'KeyD':
                    this.movement = Direction.Right;
                    break;
                case 'Space':
                    console.log(this.movement);
                    // console.log(this.gameStoped);
                    this.movement = undefined;
                    this.gameStoped = !this.gameStoped;
                    break;

                default:
                    break;
            }
        });
    }
}

import { Direction } from 'wasm-snake-game';

export class Controller {
    constructor(status = 'going') {
        this.gameStatus = status;
        this.gameStoped = false;
        this.addEvents();
    }

    addEvents() {
        document.addEventListener('keydown', ({ code }) => {
            // console.log(code);
            switch (code) {
                case 'KeyW':
                case 'ArrowUp':
                    this.movement = Direction.Up;
                    break;
                case 'KeyS':
                case 'ArrowDown':
                    this.movement = Direction.Down;
                    break;
                case 'KeyA':
                case 'ArrowLeft':
                    this.movement = Direction.Left;
                    break;
                case 'KeyD':
                case 'ArrowRight':
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

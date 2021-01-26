import { Controller } from 'stimulus';

export default class extends Controller {
    reset() {
        (this.element as HTMLFormElement).reset();
    }
}